use actix::Message;

use crate::db::implementation::Connection;
use crate::db::messages::session::CreateSession;
use crate::errors::UserError;
use crate::utils::verify_password;

use urusai_lib::models::user::User;

/// Validates a `CreateSession` message against the `Connection`, returning a `User` on successful login.
///
/// Returns `UserError::LoginError` if the provided `display_name` doesn't exist, or we can't verify the provided `password` against the user.
pub fn create(conn: &Connection, msg: &CreateSession) -> <CreateSession as Message>::Result {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use urusai_lib::schema::users::dsl::*;

    let user = users
        .filter(display_name.eq(&msg.display_name))
        .first::<User>(conn);

    match user {
        Ok(user) => {
            if verify_password(&user, &msg.password) {
                if user.email_verified {
                    Ok(user)
                } else {
                    Err(UserError::EmailNotVerified)
                }
            } else {
                Err(UserError::LoginError)
            }
        }
        Err(_) => Err(UserError::LoginError),
    }
}

#[cfg(test)]
mod tests {
    use crate::db::messages::user::{CreateUser, VerifyUser};
    use diesel::result::Error;
    use diesel::Connection;
    use dotenv::dotenv;
    use std::env;
    use urusai_lib::models::user_token::UserToken;

    use super::*;

    fn get_connection() -> crate::db::implementation::Connection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = crate::db::implementation::Database::new(database_url).unwrap();

        db.pool.get().unwrap()
    }

    fn create_user(conn: &crate::db::implementation::Connection) -> (User, UserToken) {
        let result = crate::db::implementation::user::create(
            &conn,
            CreateUser {
                display_name: "test_user".to_string(),
                email: "test@user.com".to_string(),
                password: "S3curePassw0rd!".to_string(),
            },
        );

        result.expect("Invalid user")
    }

    fn verify_user(conn: &crate::db::implementation::Connection, user: &User, token: &UserToken) {
        let _result = crate::db::implementation::user::verify(
            &conn,
            &VerifyUser {
                id: token.id,
                user_id: user.id,
            },
        );
    }

    #[test]
    fn success() {
        let conn = get_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let (user, token) = create_user(&conn);
            verify_user(&conn, &user, &token);

            let result = create(
                &conn,
                &CreateSession {
                    display_name: "test_user".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

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
            let (_user, _) = create_user(&conn);

            let result = create(
                &conn,
                &CreateSession {
                    display_name: "test_user".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

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

            let result = create(
                &conn,
                &CreateSession {
                    display_name: "test_user".to_string(),
                    password: "S3curePassword!".to_string(),
                },
            );

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

            let result = create(
                &conn,
                &CreateSession {
                    display_name: "test".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

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

            let result = create(
                &conn,
                &CreateSession {
                    display_name: "test_user".to_string(),
                    password: "' OR 1=1;--".to_string(),
                },
            );

            assert_eq!(result.err(), Some(UserError::LoginError));

            Ok(())
        });
    }
}
