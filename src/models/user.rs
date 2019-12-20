use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct Username(pub String);

#[derive(Serialize, Deserialize, Debug, ToSql, FromSql)]
pub struct Email(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: Uuid,
  pub display_name: Username,
  pub email: Email,
  pub email_verified: bool,
  #[serde(skip_serializing)]
  #[serde(default)]
  pub password_hash: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct NewUser<'a> {
  pub id: &'a Uuid,
  pub display_name: &'a str,
  pub email: &'a str,
  pub password_hash: &'a str,
}

impl std::fmt::Display for Username {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::fmt::Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
