use chrono::NaiveDateTime;
use uuid::Uuid;

use urusai_lib::models::user_token::{TokenScope, UserToken};

use crate::db::Pool;
use crate::errors::UserError;

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct VerifyUser {
  pub id: Uuid,
  pub user_id: Uuid,
}

pub async fn verify(pool: &Pool, msg: VerifyUser) -> Result<UserToken, UserError> {
  let statement = "
    SELECT
      id,
      user_id,
      scope,
      created_at,
      updated_at
    FROM user_tokens
    WHERE
      id = $1
      AND user_id = $2
      AND scope = $3
  ";

  let update_statement = "UPDATE users SET email_verified = true WHERE id = $1";

  let mut client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;
  let prepared_update_statement = client.prepare(update_statement).await?;

  let result = client
    .query_one(&prepared_statement, &[&msg.id, &msg.user_id, &TokenScope::Activation])
    .await?;

  let token = UserToken {
    id: result.try_get::<_, Uuid>(0)?,
    user_id: result.try_get::<_, Uuid>(1)?,
    scope: result.try_get::<_, TokenScope>(2)?,
    created_at: result.try_get::<_, NaiveDateTime>(3)?,
    updated_at: result.try_get::<_, NaiveDateTime>(4)?,
  };

  client.execute(&prepared_update_statement, &[&msg.user_id]).await?;

  Ok(token)
}
