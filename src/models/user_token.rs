use crate::schema::user_tokens;

use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(DbEnum, Debug, Serialize, Deserialize)]
pub enum TokenScope {
    Activation,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct UserToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub scope: TokenScope,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "user_tokens"]
pub struct NewUserToken<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub scope: &'a TokenScope,
}
