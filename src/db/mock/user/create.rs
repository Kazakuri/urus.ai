use actix::Message;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::db::messages::user::CreateUser;
use urusai_lib::models::user::{ User, NewUser };
use crate::errors::UserError;

/// Creates a new user from a `CreateUser` message, returning the newly created `User`.
pub fn create(msg: CreateUser) -> <CreateUser as Message>::Result {
  if msg.display_name == "existing_user" {
    return Err(UserError::DuplicateValue {
      field: "Display Name".to_string()
    });
  }

  Ok(User {
    id: Uuid::parse_str("00000000000000000000000000000002").expect("Invalid UUID provided"),
    display_name: msg.display_name,
    email: msg.email,
    email_verified: true,
    password_hash: "some_hash".to_string(),
    created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    updated_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
  })
}
