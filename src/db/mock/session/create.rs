use actix::Message;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::db::messages::session::CreateSession;
use crate::errors::UserError;
use urusai_lib::models::user::User;

/// Validates a `CreateSession` message, returning a `User` on successful login.
///
/// Returns `UserError::LoginError` if the provided `display_name` doesn't exist, or we can't verify the provided `password` against the user.
pub fn create(msg: &CreateSession) -> <CreateSession as Message>::Result {
    if &msg.display_name == "test_account_1" {
        return match msg.password.as_ref() {
            "password" => Ok(User {
                id: Uuid::parse_str("00000000000000000000000000000001")
                    .expect("Invalid UUID provided"),
                display_name: "test_user".to_string(),
                email: "test@user.com".to_string(),
                email_verified: true,
                password_hash: "".to_string(),
                created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
                updated_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            }),
            _ => Err(UserError::LoginError),
        };
    }

    Err(UserError::LoginError)
}
