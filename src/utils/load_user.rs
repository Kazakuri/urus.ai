use uuid::Uuid;

use urusai_lib::models::user::User;

use crate::db::user::ReadUser;
use crate::db::Pool;

pub async fn load_user(identity: Option<String>, db: &Pool) -> Option<User> {
  if let Some(id) = identity {
    let uuid = Uuid::parse_str(&id);

    if uuid.is_err() {
      return None;
    }

    let user_info = ReadUser {
      id: uuid.expect("Invalid UUID provided"),
    };

    let user = crate::db::user::read(db, user_info).await;

    return match user {
      Ok(u) => Some(u),
      Err(_) => None,
    };
  }

  None
}
