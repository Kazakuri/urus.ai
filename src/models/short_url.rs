use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortURL {
  pub id: Uuid,
  pub slug: String,
  pub url: String,
  pub visits: i64,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct NewShortURL<'a> {
  pub id: &'a Uuid,
  pub slug: &'a str,
  pub url: &'a str,
}
