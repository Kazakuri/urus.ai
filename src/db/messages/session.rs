use actix::Message;

use urusai_codegen::DbMessage;
use actix::Handler;
use crate::db::DbExecutor;

use crate::errors::UserError;
use urusai_lib::models::user::User;

/// Message to create a new session by logging in with a `display_name` and `password`
#[derive(Deserialize, Serialize, DbMessage)]
pub struct CreateSession {
  /// The display_name of the account the user requests a session for.
  pub display_name: String,

  /// The password for the account of the requested display_name.
  pub password: String,
}

impl Message for CreateSession {
  type Result = Result<User, UserError>;
}
