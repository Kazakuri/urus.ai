use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub enum TokenScope {
  #[postgres(name = "activation")]
  Activation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
  pub id: Uuid,
  pub user_id: Uuid,
  pub scope: TokenScope,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct NewUserToken<'a> {
  pub id: &'a Uuid,
  pub user_id: &'a Uuid,
  pub scope: &'a TokenScope,
}
