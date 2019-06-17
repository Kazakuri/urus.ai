use actix::Message;
use sodiumoxide::crypto::pwhash::argon2id13;
use bcrypt;

use crate::db::messages::session::CreateSession;
use crate::db::implementation::Connection;
use urusai_lib::models::user::User;
use crate::errors::UserError;

/// Validates a `CreateSession` message against the `Connection`, returning a `User` on successful login.
///
/// Returns `UserError::LoginError` if the provided `display_name` doesn't exist, or we can't verify the provided `password` against the user.
pub fn create(conn: &Connection, msg: &CreateSession) -> <CreateSession as Message>::Result {
  use urusai_lib::schema::users::dsl::*;
  use diesel::RunQueryDsl;
  use diesel::QueryDsl;
  use diesel::ExpressionMethods;

  let user = users
    .filter(display_name.eq(&msg.display_name))
    .first::<User>(conn);

  match user {
    Ok(user) => {
      let mut bytes = user.password_hash.as_bytes().to_vec();

      if !bytes.starts_with(&b"$argon2id$"[..]) {
        return match bcrypt::verify(&msg.password, &user.password_hash) {
          Ok(ok) => return if ok { Ok(user) } else { Err(UserError::LoginError) },
          Err(_) => Err(UserError::LoginError),
        };
      }

      // argon2 passwords are padded by null bytes and Postgres can't store null bytes, so we strip them
      // Here we re-pad the loaded password with null bytes to reverse the stripping we did when we generated the hash.
      bytes.resize(128, 0x00);

      let hash = argon2id13::HashedPassword::from_slice(&bytes[..])
        .expect("Could not resolve password_hash as a valid argon2id hash");

      if argon2id13::pwhash_verify(&hash, &msg.password.as_bytes()[..]) {
        if user.email_verified {
          Ok(user)
        } else {
          Err(UserError::EmailNotVerified)
        }
      } else {
        Err(UserError::LoginError)
      }
    },
    Err(_) => Err(UserError::LoginError),
  }
}

#[cfg(test)]
mod tests {
  use diesel::result::Error;
  use diesel::Connection;
  use std::env;
  use dotenv::dotenv;
  use crate::db::messages::user::{ CreateUser, VerifyUser };
  use urusai_lib::models::user_token::UserToken;

  use super::*;

  fn get_connection() -> crate::db::implementation::Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = crate::db::implementation::Database::new(database_url).unwrap();

    db.pool.get().unwrap()
  }

  fn create_user(conn: &crate::db::implementation::Connection) -> (User, UserToken) {
      let result = crate::db::implementation::user::create(&conn, CreateUser {
        display_name: "test_user".to_string(),
        email: "test@user.com".to_string(),
        password: "S3curePassw0rd!".to_string(),
      });

      result.expect("Invalid user")
  }

  fn verify_user(conn: &crate::db::implementation::Connection, user: &User, token: &UserToken) {
    let result = crate::db::implementation::user::verify(&conn, &VerifyUser {
      id: token.id,
      user_id: user.id,
    });
  }

  #[test]
  fn success() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, token) = create_user(&conn);
      verify_user(&conn, &user, &token);

      let result = create(&conn, &CreateSession {
        display_name: "test_user".to_string(),
        password: "S3curePassw0rd!".to_string(),
      });

      let user = result.expect("Invalid user");

      assert_eq!(user.display_name, "test_user");
      assert_eq!(user.email, "test@user.com");
      assert_ne!(user.password_hash, "S3curePassw0rd!");

      Ok(())
    });
  }


  #[test]
  fn fail_not_verified() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, _) = create_user(&conn);

      let result = create(&conn, &CreateSession {
        display_name: "test_user".to_string(),
        password: "S3curePassw0rd!".to_string(),
      });

      assert_eq!(result.err(), Some(UserError::EmailNotVerified));

      Ok(())
    });
  }

  #[test]
  fn fail_bad_password() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, token) = create_user(&conn);
      verify_user(&conn, &user, &token);

      let result = create(&conn, &CreateSession {
        display_name: "test_user".to_string(),
        password: "S3curePassword!".to_string(),
      });

      assert_eq!(result.err(), Some(UserError::LoginError));

      Ok(())
    });
  }

  #[test]
  fn fail_bad_display_name() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, token) = create_user(&conn);
      verify_user(&conn, &user, &token);

      let result = create(&conn, &CreateSession {
        display_name: "test".to_string(),
        password: "S3curePassw0rd!".to_string(),
      });

      assert_eq!(result.err(), Some(UserError::LoginError));

      Ok(())
    });
  }

  #[test]
  fn fail_sql_injection() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, token) = create_user(&conn);
      verify_user(&conn, &user, &token);

      let result = create(&conn, &CreateSession {
        display_name: "test_user".to_string(),
        password: "' OR 1=1;--".to_string(),
      });

      assert_eq!(result.err(), Some(UserError::LoginError));

      Ok(())
    });
  }
}
