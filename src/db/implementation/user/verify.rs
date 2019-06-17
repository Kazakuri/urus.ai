use actix::Message;

use crate::db::messages::user::VerifyUser;
use crate::db::implementation::Connection;
use urusai_lib::models::user_token::{ UserToken, TokenScope };
use crate::errors::UserError;

/// Verifies a user's email from a `VerifyUser` message
pub fn verify(conn: &Connection, msg: &VerifyUser) -> <VerifyUser as Message>::Result {
  use urusai_lib::schema::user_tokens::dsl::*;
  use urusai_lib::schema::users::dsl::*;
  use urusai_lib::schema::user_tokens;

  use diesel::RunQueryDsl;
  use diesel::QueryDsl;
  use diesel::ExpressionMethods;

  let user = user_tokens
    .filter(user_tokens::dsl::id.eq(&msg.id))
    .filter(user_id.eq(&msg.user_id))
    .filter(scope.eq(TokenScope::Activation))
    .first::<UserToken>(conn);

  match user {
    Ok(user) => {
      // Verify the user's email
      let user_update = diesel::update(users.find(&msg.user_id))
        .set(email_verified.eq(true))
        .execute(conn);

      if user_update.is_err() {
        warn!("Could not verify user email: {}", user.id);
      }

      Ok(user)
    },
    Err(_) => Err(UserError::NotFound),
  }
}

#[cfg(test)]
mod tests {
  use diesel::result::Error;
  use diesel::Connection;
  use std::env;
  use dotenv::dotenv;
  use uuid::Uuid;
  use crate::db::messages::user::CreateUser;
  use urusai_lib::models::user::User;

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

  fn email_verified(conn: &crate::db::implementation::Connection, user_id: &Uuid) -> bool {
    use urusai_lib::schema::users::dsl::*;
    use diesel::RunQueryDsl;
    use diesel::QueryDsl;
    use diesel::ExpressionMethods;

    users
      .filter(id.eq(user_id))
      .first::<User>(conn)
      .expect("Unable to find URL")
      .email_verified
  }

  #[test]
  fn success() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, token) = create_user(&conn);

      verify(&conn, &VerifyUser {
        id: token.id,
        user_id: user.id,
      }).expect("Failed to verify user");

      assert!(email_verified(&conn, &user.id));

      Ok(())
    });
  }

  // TODO: fail unknown user / unknown token

  #[test]
  fn fail_unknown() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = verify(&conn, &VerifyUser {
        id: Uuid::nil(),
        user_id: Uuid::nil(),
      });

      assert!(result.is_err());
      assert_eq!(result.err(), Some(UserError::NotFound));

      Ok(())
    });
  }
}
