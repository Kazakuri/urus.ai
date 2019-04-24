use actix::Message;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::db::messages::url::CreateURL;
use urusai_lib::models::short_url::{ ShortURL, NewShortURL };
use crate::errors::UserError;

/// Creates a new association to a url from a `CreateURL`, returning the created `ShortURL` association.
pub fn create(msg: CreateURL) -> <CreateURL as Message>::Result {
  if msg.url == "https://example.com" {
    return Ok(ShortURL {
      id: Uuid::parse_str("00000000000000000000000000000002").expect("Invalid UUID provided"),
      user_id: Some(Uuid::parse_str("00000000000000000000000000000001").expect("Invalid UUID provided")),
      slug: match msg.slug {
        Some(slug) => slug,
        None => "test_slug".to_string()
      },
      url: "https://example.com".to_string(),
      visits: 0,
      created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
      updated_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
    })
  }
  Err(UserError::InvalidCharactersInURL)
}
