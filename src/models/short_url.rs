use crate::schema::urls;

use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::models::user::User;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name="urls"]
pub struct ShortURL {
  pub id: Uuid,
  pub user_id: Option<Uuid>,
  pub slug: String,
  pub url: String,
  pub visits: i64,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name="urls"]
pub struct NewShortURL<'a> {
  pub id: &'a Uuid,
  pub user_id: Option<Uuid>,
  pub slug: &'a str,
  pub url: &'a str,
}
