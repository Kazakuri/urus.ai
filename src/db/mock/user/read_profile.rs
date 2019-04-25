use actix::Message;

use crate::db::messages::user::ReadUserProfile;
use urusai_lib::models::user::{ User };
use urusai_lib::models::short_url::{ ShortURL };
use crate::errors::UserError;

/// Resolves a user's profile from a `ReadUserProfile` message
pub fn read_profile( msg: &ReadUserProfile) -> <ReadUserProfile as Message>::Result {
  Err(UserError::NotFound)
}