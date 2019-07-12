use actix::Message;

use crate::db::messages::user::ChangeUserPassword;
use crate::errors::UserError;

/// Changes a user's password from a `ChangeUserPassword` message
pub fn password_change(_msg: &ChangeUserPassword) -> <ChangeUserPassword as Message>::Result {
    Err(UserError::InternalError)
}
