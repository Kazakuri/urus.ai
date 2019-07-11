use actix::Message;
use uuid::Uuid;

use crate::db::DbExecutor;
use actix::Handler;
use urusai_codegen::DbMessage;

use crate::errors::UserError;
use urusai_lib::models::short_url::ShortURL;
use urusai_lib::models::user::User;
use urusai_lib::models::user_token::UserToken;

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
    /// The verification token for the user.
    pub id: Uuid,
    /// The ID of the user to verify.
    pub user_id: Uuid,
}

///
#[derive(Deserialize, Serialize, DbMessage)]
pub struct ChangeUserPassword {
    /// The ID of the user being updated.
    pub id: Uuid,

    /// 
    pub new_password: String,

    /// 
    pub current_password: String,

    /// 
    pub confirm_password: String,
}

impl Message for CreateUser {
    type Result = Result<(User, UserToken), UserError>;
}

impl Message for ReadUserProfile {
    type Result = Result<(User, Vec<ShortURL>, i64), UserError>;
}

impl Message for ReadUser {
    type Result = Result<User, UserError>;
}

impl Message for VerifyUser {
    type Result = Result<UserToken, UserError>;
}

impl Message for ChangeUserPassword {
    type Result = Result<User, UserError>;
}
