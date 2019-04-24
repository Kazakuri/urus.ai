use actix::Message;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::db::messages::user::ReadUser;
use urusai_lib::models::user::{ User };
use crate::errors::UserError;

/// Resolves a user from a `ReadUser` message
pub fn read(msg: &ReadUser) -> <ReadUser as Message>::Result {
  let uuid = Uuid::parse_str("00000000000000000000000000000002")
    .expect("Failed to parse UUID");

  if msg.id == uuid {
    return Ok(User {
      id: Uuid::parse_str("00000000000000000000000000000002").expect("Invalid UUID provided"),
      display_name: "test_user".to_string(),
      email: "test@user.com".to_string(),
      email_verified: true,
      password_hash: "some_hash".to_string(),
      created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
      updated_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    });
  }

  Err(UserError::NotFound)
}
