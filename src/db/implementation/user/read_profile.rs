use actix::Message;

use crate::db::messages::user::ReadUserProfile;
use crate::db::implementation::Connection;
use urusai_lib::models::user::{ User };
use urusai_lib::models::short_url::{ ShortURL };
use crate::errors::UserError;

/// The number of entries per page
const COUNT_PER_PAGE: i64 = 20;

/// Resolves a user's profile from a `ReadUserProfile` message
pub fn read_profile(conn: &Connection, msg: &ReadUserProfile) -> <ReadUserProfile as Message>::Result {
  use urusai_lib::schema::users::dsl::*;
  use diesel::RunQueryDsl;
  use diesel::QueryDsl;
  use diesel::ExpressionMethods;
  use diesel::BelongingToDsl;
  use diesel::dsl::count;

  if msg.page < 1 {
    return Err(UserError::NotFound);
  }

  let user = users
    .filter(id.eq(&msg.id))
    .first::<User>(conn);

  match user {
    Ok(user) => {
      let urls = ShortURL::belonging_to(&user)
        .order(urusai_lib::schema::urls::columns::updated_at.desc())
        .offset((msg.page - 1) * COUNT_PER_PAGE)
        .limit(COUNT_PER_PAGE)
        .load::<ShortURL>(conn)
        .expect("Error loading urls");

      let count: i64 = ShortURL::belonging_to(&user)
        .select(count(urusai_lib::schema::urls::columns::id))
        .first(conn)
        .expect("Error loading urls");

      Ok((user, urls, (count / COUNT_PER_PAGE) + 1))
    },
    Err(_) => Err(UserError::NotFound),
  }
}

#[cfg(test)]
mod tests {
  use diesel::result::Error;
  use diesel::Connection;
  use std::env;
  use dotenv::dotenv;
  use crate::db::messages::user::CreateUser;
  use crate::db::messages::url::CreateURL;

  use super::*;

  fn get_connection() -> crate::db::implementation::Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = crate::db::implementation::Database::new(database_url).unwrap();

    db.pool.get().unwrap()
  }

  fn create_user(conn: &crate::db::implementation::Connection) -> User {
    let result = crate::db::implementation::user::create(&conn, CreateUser {
      display_name: "test_user".to_string(),
      email: "test@user.com".to_string(),
      password: "S3curePassw0rd!".to_string(),
    });

    result.expect("Invalid user")
  }

  fn create_url(conn: &crate::db::implementation::Connection, user: &User) -> ShortURL {
    let result = crate::db::implementation::url::create(&conn, CreateURL {
      url: "example.com".to_string(),
      slug: None,
      user_id: Some(user.id.to_string()),
    });

    result.expect("Invalid url")
  }

  #[test]
  fn success_empty() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let user = create_user(&conn);

      let result = read_profile(&conn, &ReadUserProfile {
        id: user.id,
        page: 1,
      });

      let (owner, urls, page_count) = result.expect("Failed to load profile");

      assert_eq!(user.id, owner.id);
      assert_eq!(urls.len(), 0);
      assert_eq!(page_count, 1);

      Ok(())
    });
  }

  #[test]
  fn success_single_url() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let user = create_user(&conn);
      let url = create_url(&conn, &user);

      let result = read_profile(&conn, &ReadUserProfile {
        id: user.id,
        page: 1,
      });

      let (owner, urls, page_count) = result.expect("Failed to load profile");

      assert_eq!(user.id, owner.id);
      assert_eq!(urls.len(), 1);
      assert_eq!(page_count, 1);

      if let Some(short_url) = urls.first() {
        assert_eq!(url.url, short_url.url);
        assert_eq!(url.slug, short_url.slug);
      }

      Ok(())
    });
  }

  #[test]
  fn success_nonexistent_page() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let user = create_user(&conn);
      let url = create_url(&conn, &user);

      let result = read_profile(&conn, &ReadUserProfile {
        id: user.id,
        page: 2,
      });

      let (owner, urls, page_count) = result.expect("Failed to load profile");

      assert_eq!(user.id, owner.id);
      assert_eq!(urls.len(), 0);
      assert_eq!(page_count, 1);

      Ok(())
    });
  }

  #[test]
  fn success_second_page() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let user = create_user(&conn);

      for _ in 0..(COUNT_PER_PAGE + 6) {
        create_url(&conn, &user);
      }

      let result = read_profile(&conn, &ReadUserProfile {
        id: user.id,
        page: 2,
      });

      let (owner, urls, page_count) = result.expect("Failed to load profile");

      assert_eq!(user.id, owner.id);
      assert_eq!(urls.len(), 6);
      assert_eq!(page_count, 2);

      Ok(())
    });
  }

  #[test]
  fn fail_invalid_page() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let user = create_user(&conn);

      let result = read_profile(&conn, &ReadUserProfile {
        id: user.id,
        page: 0,
      });

      assert!(result.is_err());
      assert_eq!(result.err(), Some(UserError::NotFound));

      Ok(())
    });
  }
}
