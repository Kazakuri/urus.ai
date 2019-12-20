use chrono::NaiveDateTime;
use uuid::Uuid;

use urusai_lib::models::user::{Email, User, Username};

use crate::db::Pool;
use crate::errors::UserError;

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct ReadUser {
  pub id: Uuid,
}

pub async fn read(pool: &Pool, msg: ReadUser) -> Result<User, UserError> {
  let statement = "
    SELECT
      id,
      display_name,
      email,
      email_verified,
      password_hash,
      created_at,
      updated_at
    FROM users
    WHERE id = $1
  ";

  let mut client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;

  debug!("Looking for {}", msg.id);

  let row = client.query_one(&prepared_statement, &[&msg.id]).await?;

  let user = User {
    id: row.try_get::<_, Uuid>(0)?,
    display_name: row.try_get::<_, Username>(1)?,
    email: row.try_get::<_, Email>(2)?,
    email_verified: row.try_get::<_, bool>(3)?,
    password_hash: row.try_get::<_, String>(4)?,
    created_at: row.try_get::<_, NaiveDateTime>(5)?,
    updated_at: row.try_get::<_, NaiveDateTime>(6)?,
  };

  Ok(user)
}
