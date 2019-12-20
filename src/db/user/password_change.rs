use chrono::NaiveDateTime;
use uuid::Uuid;

use urusai_lib::models::user::{Email, User, Username};

use crate::db::Pool;
use crate::errors::UserError;
use crate::utils::{validate_and_hash_password, verify_password};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeUserPassword {
  // We don't want the user_id to be deserializable from serde
  // Otherwise anybody could try to change any user's password
  #[serde(skip)]
  pub id: Option<Uuid>,
  pub new_password: String,
  pub current_password: String,
  pub confirm_password: String,
}

pub async fn password_change(pool: &Pool, msg: ChangeUserPassword) -> Result<User, UserError> {
  // TODO: Why isn't this in the other function?
  if msg.new_password != msg.confirm_password {
    return Err(UserError::LoginError);
  }

  let statement = "
    SELECT password_hash FROM users
    WHERE id = $1
  ";

  let update_statement = "
    UPDATE users
    SET password_hash = $1
    WHERE id = $2
    RETURNING id, display_name, email, email_verified, password_hash, created_at, updated_at
  ";

  let mut client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;
  let prepared_update_statement = client.prepare(update_statement).await?;

  let user = client.query_one(&prepared_statement, &[&msg.id]).await?;

  let hash = user.try_get::<_, String>(0)?;

  if verify_password(&hash, &msg.current_password) {
    let hash = validate_and_hash_password(msg.new_password.clone())?;

    let row = client.query_one(&prepared_update_statement, &[&hash, &msg.id]).await?;

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
  } else {
    Err(UserError::LoginError)
  }
}
