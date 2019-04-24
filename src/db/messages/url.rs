use actix::Message;

use urusai_codegen::DbMessage;
use actix::Handler;
use crate::db::DbExecutor;

use crate::errors::UserError;
use urusai_lib::models::short_url::ShortURL;

/// Message to create a new shortened URL of an existing `url` with an optional `slug` and `user_id`
#[derive(Deserialize, Serialize, DbMessage)]
pub struct CreateURL {
  /// The full URL that is being shortened.
  pub url: String,

  /// An optional string to use instead of a randomly generated one for the URL.
  pub slug: Option<String>,

  // We don't want the user_id to be deserializable from serde
  // Otherwise anybody could create messages from any user
  #[serde(skip)]
  /// An optional ID for a User to accociate the ShortURL with.
  pub user_id: Option<String>,
}

/// Message to expand a shortened URL `slug` into the original URL
#[derive(Deserialize, Serialize, DbMessage)]
pub struct ReadURL {
  /// The shortened URL slug to expand into the original URL.
  pub slug: String,
}

impl Message for CreateURL {
  type Result = Result<ShortURL, UserError>;
}

impl Message for ReadURL {
  type Result = Result<ShortURL, UserError>;
}
