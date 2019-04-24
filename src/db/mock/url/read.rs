use actix::Message;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::db::messages::url::ReadURL;
use urusai_lib::models::short_url::{ ShortURL };
use crate::errors::UserError;

/// Resolves a `ReadURL` message from the shortened slug to the whole `ShortURL` instance.
pub fn read(msg: &ReadURL) -> <ReadURL as Message>::Result {
  if msg.slug == "example" {
    return Ok(ShortURL {
      id: Uuid::parse_str("00000000000000000000000000000002").expect("Invalid UUID provided"),
      user_id: None,
      slug: "example".to_string(),
      url: "https://example.com".to_string(),
      visits: 0,
      created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
      updated_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
    });
  }

  Err(UserError::NotFound)
}
