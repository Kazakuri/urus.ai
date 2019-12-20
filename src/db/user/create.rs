use chrono::NaiveDateTime;
use uuid::Uuid;

use urusai_lib::models::user::{Email, User, Username};
use urusai_lib::models::user_token::{TokenScope, UserToken};

use crate::db::Pool;
use crate::errors::UserError;
use crate::utils::validate_and_hash_password;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUser {
  pub display_name: String,

  pub email: String,

  pub password: String,
}

pub async fn create(pool: &Pool, msg: CreateUser) -> Result<(User, UserToken), UserError> {
  let statement = "
    INSERT INTO users (
      display_name,
      email,
      password_hash
    ) VALUES ($1, $2, $3)
    RETURNING id, display_name, email, email_verified, password_hash, created_at, updated_at
  ";

  let token_statement = "
    INSERT INTO user_tokens (
      user_id,
      scope
    ) VALUES ($1, $2)
    RETURNING id, user_id, scope, created_at, updated_at
  ";

  let mut client = pool.get().await?;

  let prepared_statement = client.prepare(statement).await?;
  let prepared_token_statement = client.prepare(token_statement).await?;

  let password_hash = validate_and_hash_password(msg.password)?;

  debug!("Creating user!");

  let result = client
    .query_one(
      &prepared_statement,
      &[&Username(msg.display_name), &Email(msg.email), &password_hash],
    )
    .await?;

  let user = User {
    id: result.try_get::<_, Uuid>(0)?,
    display_name: result.try_get::<_, Username>(1)?,
    email: result.try_get::<_, Email>(2)?,
    email_verified: result.try_get::<_, bool>(3)?,
    password_hash: result.try_get::<_, String>(4)?,
    created_at: result.try_get::<_, NaiveDateTime>(5)?,
    updated_at: result.try_get::<_, NaiveDateTime>(6)?,
  };

  debug!("Creating user token!");

  let result = client
    .query_one(&prepared_token_statement, &[&user.id, &TokenScope::Activation])
    .await?;

  let token = UserToken {
    id: result.try_get::<_, Uuid>(0)?,
    user_id: result.try_get::<_, Uuid>(1)?,
    scope: result.try_get::<_, TokenScope>(2)?,
    created_at: result.try_get::<_, NaiveDateTime>(3)?,
    updated_at: result.try_get::<_, NaiveDateTime>(4)?,
  };

  Ok((user, token))
}
