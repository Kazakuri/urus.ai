use actix::Message;
use uuid::Uuid;

use urusai_codegen::DbMessage;
use actix::Handler;
use crate::db::DbExecutor;

use crate::errors::UserError;
use urusai_lib::models::user::User;
use urusai_lib::models::short_url::ShortURL;

/// Message to create a new user by logging in with the supplied profile information
#[derive(Deserialize, Serialize, DbMessage)]
pub struct CreateUser {
  /// The new user's requested display name.
  pub display_name: String,

  /// The new user's associated e-mail address.
  pub email: String,

  /// The new user's chosen password.
  pub password: String,
}

/// Message to read a user's complete profile, including a list of URLs they've created
#[derive(Deserialize, Serialize, DbMessage)]
pub struct ReadUserProfile {
  /// The ID of the user whose profile is being requested.
  pub id: Uuid,

  /// The offset of the URLs to load
  pub page: i64,
}

/// Message to read a user and get their account information from their ID
#[derive(Deserialize, Serialize, DbMessage)]
pub struct ReadUser {
  /// The ID of the user being requested.
  pub id: Uuid,
}

/// Message to verify a new user's email address
#[derive(Deserialize, Serialize, DbMessage)]
pub struct VerifyUser {
  /// The ID of the user to verify.
  pub id: Uuid,
}

impl Message for CreateUser {
  type Result = Result<User, UserError>;
}

impl Message for ReadUserProfile {
  type Result = Result<(User, Vec<ShortURL>, i64), UserError>;
}

impl Message for ReadUser {
  type Result = Result<User, UserError>;
}

impl Message for VerifyUser {
  type Result = Result<User, UserError>;
}
