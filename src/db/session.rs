use chrono::NaiveDateTime;
use uuid::Uuid;

use urusai_lib::models::user::{Email, User, Username};

use crate::db::Pool;
use crate::errors::UserError;
use crate::utils::verify_password;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateSession {
  pub display_name: String,

  pub password: String,
}

pub async fn create(pool: &Pool, msg: CreateSession) -> Result<User, UserError> {
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
    WHERE display_name = $1
  ";

  let mut client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;

  let row = client.query_one(&prepared_statement, &[&msg.display_name]).await?;

  let user = User {
    id: row.get::<_, Uuid>(0),
    display_name: row.get::<_, Username>(1),
    email: row.get::<_, Email>(2),
    email_verified: row.get::<_, bool>(3),
    password_hash: row.get::<_, String>(4),
    created_at: row.get::<_, NaiveDateTime>(5),
    updated_at: row.get::<_, NaiveDateTime>(6),
  };

  if verify_password(&user.password_hash, &msg.password) {
    if user.email_verified {
      Ok(user)
    } else {
      Err(UserError::EmailNotVerified)
    }
  } else {
    Err(UserError::LoginError)
  }
}
