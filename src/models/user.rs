use crate::schema::users;

use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub display_name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}
