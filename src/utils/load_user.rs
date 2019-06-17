use futures::future::Future;
use uuid::Uuid;
use actix::Addr;

use crate::db::DbExecutor;
use crate::db::messages::user::ReadUser;
use urusai_lib::models::user::User;

/// Loads a user from a UUID, returning the associated user.
pub fn load_user(identity: Option<String>, db: &Addr<DbExecutor>) -> Option<User> {
  if let Some(id) = identity {
    let uuid = Uuid::parse_str(&id);

    if uuid.is_err() {
      return None;
    }

    let user_info = ReadUser {
      id: uuid.expect("Invalid UUID provided")
    };

    let fut = db.send(user_info)
      .timeout(std::time::Duration::new(5, 0))
      .and_then(move |res| {
        match res {
          Ok(user) => {
            futures::future::ok(Some(user))
          },
          Err(_e) => {
            futures::future::ok(None)
          }
        }
      });

    return match fut.wait() {
      Ok(res) => res,
      Err(_) => None
    }
  }

  None
}

#[cfg(test)]
mod test {
  use actix::prelude::SyncArbiter;

  use super::*;

  use crate::db::{ DbExecutor, mock };

  #[test]
  fn success_no_user() {
    let sys = actix_rt::System::new("urusai_test");

    let db = SyncArbiter::start(1, move || {
      DbExecutor(mock().expect("Failed to get DB instance"))
    });

    let result = load_user(None, &db);

    assert!(result.is_none());
  }

  #[test]
  fn success_with_user() {
    let sys = actix_rt::System::new("urusai_test");

    let db = SyncArbiter::start(1, move || {
      DbExecutor(mock().expect("Failed to get DB instance"))
    });

    let result = load_user(Some("00000000000000000000000000000002".to_string()), &db);

    assert!(result.is_some());

    match result {
      Some(user) => {
        assert_eq!(user.display_name, "test_user".to_string());
      },
      None => panic!("Expected user")
    };
  }
}
