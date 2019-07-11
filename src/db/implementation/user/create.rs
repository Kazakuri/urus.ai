use actix::Message;
use sodiumoxide::crypto::pwhash::argon2id13;
use uuid::Uuid;

use crate::db::implementation::Connection;
use crate::db::messages::user::CreateUser;
use crate::errors::UserError;
use crate::utils::validate_and_hash_password;
use urusai_lib::models::user::{NewUser, User};
use urusai_lib::models::user_token::{NewUserToken, TokenScope, UserToken};

/// Creates a new user from a `CreateUser` message within the `Connection`, returning the newly created `User`.
///
/// When a user is created, an e-mail will be sent to the email address defined in msg.email
pub fn create(conn: &Connection, msg: CreateUser) -> <CreateUser as Message>::Result {
    use urusai_lib::schema::user_tokens;
    use urusai_lib::schema::user_tokens::dsl::*;
    use urusai_lib::schema::users;
    use urusai_lib::schema::users::dsl::*;

    use diesel::result::{DatabaseErrorKind, Error};
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    let uuid = Uuid::new_v4();
    
    

    let result = match validate_and_hash_password(msg.password) {
        Ok(hash) => {
            let mut new_user = NewUser {
                id: &uuid,
                display_name: &msg.display_name,
                email: &msg.email,
                password_hash: &hash,
            };

            diesel::insert_into(users).values(&new_user).execute(conn)
        },
        Err(e) => return Err(e)
    };

    match result {
        Err(e) => match e {
            Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => Err(UserError::DuplicateValue {
                    field: info
                        .constraint_name()
                        .expect("Missing column for violation")
                        .to_string(),
                }),
                DatabaseErrorKind::ForeignKeyViolation => Err(UserError::UnknownValue {
                    field: info
                        .constraint_name()
                        .expect("Missing column for violation")
                        .to_string(),
                }),
                _ => match info.constraint_name() {
                    Some(field) => Err(UserError::InvalidValue {
                        field: field.to_string(),
                    }),
                    None => {
                        error!("Expected some field for a constraint error but found none!");
                        Err(UserError::InternalError)
                    }
                },
            },
            _ => {
                error!("Could not insert into the database!");
                Err(UserError::InternalError)
            }
        },
        Ok(_) => {
            let token = Uuid::new_v4();

            let user_token = NewUserToken {
                id: &token,
                user_id: &uuid,
                scope: &TokenScope::Activation,
            };

            let result = diesel::insert_into(user_tokens)
                .values(&user_token)
                .execute(conn);

            if result.is_err() {
                error!("Could not create a new user token!");
                return Err(UserError::InternalError);
            }

            let item = users
                .filter(users::dsl::id.eq(&uuid))
                .first::<User>(conn)
                .expect("Error loading user");

            let token = user_tokens
                .filter(user_tokens::dsl::id.eq(&token))
                .first::<UserToken>(conn)
                .expect("Error loading user token");

            Ok((item, token))
        }
    }
}

#[cfg(test)]
mod tests {
    use diesel::result::Error;
    use diesel::Connection;
    use dotenv::dotenv;
    use std::env;

    use super::*;

    fn get_connection() -> crate::db::implementation::Connection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = crate::db::implementation::Database::new(database_url).unwrap();

        db.pool.get().unwrap()
    }

    #[test]
    fn success() {
        let conn = get_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let result = create(
                &conn,
                CreateUser {
                    display_name: "test_user".to_string(),
                    email: "test@user.com".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

            let (user, _) = result.expect("Invalid User");

            assert_eq!(user.display_name, "test_user");
            assert_ne!(user.password_hash, "S3curePassw0rd!");
            assert_eq!(user.email, "test@user.com");
            assert_eq!(user.email_verified, false);

            Ok(())
        });
    }

    #[test]
    fn fail_duplicate_display_name() {
        let conn = get_connection();

        conn.test_transaction::<_, Error, _>(|| {
            create(
                &conn,
                CreateUser {
                    display_name: "test_user".to_string(),
                    email: "test@user.com".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            )
            .expect("Failed to create user");

            let result = create(
                &conn,
                CreateUser {
                    display_name: "test_user".to_string(),
                    email: "test2@user.com".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

            assert!(result.is_err());
            assert_eq!(
                result.err(),
                Some(UserError::DuplicateValue {
                    field: "Display Name".to_string()
                })
            );

            Ok(())
        });
    }

    #[test]
    fn fail_duplicate_email() {
        let conn = get_connection();

        conn.test_transaction::<_, Error, _>(|| {
            create(
                &conn,
                CreateUser {
                    display_name: "test_user".to_string(),
                    email: "test@user.com".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            )
            .expect("Failed to create user");

            let result = create(
                &conn,
                CreateUser {
                    display_name: "test2_user".to_string(),
                    email: "test@user.com".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

            assert!(result.is_err());
            assert_eq!(
                result.err(),
                Some(UserError::DuplicateValue {
                    field: "Email".to_string()
                })
            );

            Ok(())
        });
    }

    #[test]
    fn fail_invalid_display_name() {
        let conn = get_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let result = create(
                &conn,
                CreateUser {
                    display_name: "t e s t".to_string(),
                    email: "test@user.com".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

            assert!(result.is_err());
            assert_eq!(
                result.err(),
                Some(UserError::InvalidValue {
                    field: "Display Name".to_string()
                })
            );

            Ok(())
        });
    }

    #[test]
    fn fail_invalid_email() {
        let conn = get_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let result = create(
                &conn,
                CreateUser {
                    display_name: "test2_user".to_string(),
                    email: "test".to_string(),
                    password: "S3curePassw0rd!".to_string(),
                },
            );

            assert!(result.is_err());
            assert_eq!(
                result.err(),
                Some(UserError::InvalidValue {
                    field: "Email".to_string()
                })
            );

            Ok(())
        });
    }
}
