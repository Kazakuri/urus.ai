use actix::Message;

use crate::db::messages::user::VerifyUser;
use urusai_lib::models::user::{ User };
use crate::errors::UserError;

/// Verfies a user's email from a `VerifyUser` message
pub fn verify(msg: &VerifyUser) -> <VerifyUser as Message>::Result {
  Err(UserError::NotFound)
}
