use actix::Message;

use crate::db::messages::url::ReadURL;
use crate::db::implementation::Connection;
use urusai_lib::models::short_url::{ ShortURL };
use crate::errors::UserError;

/// Resolves a `ReadURL` message from the shortened slug to the whole `ShortURL` instance.
///
/// Calling this function will update the resolved `ShortURL` with an incremented visit count.
pub fn read(conn: &Connection, msg: &ReadURL) -> <ReadURL as Message>::Result {
  use urusai_lib::schema::urls::dsl::*;
  use diesel::RunQueryDsl;
  use diesel::QueryDsl;
  use diesel::ExpressionMethods;

  let item = urls
    .filter(slug.eq(&msg.slug))
    .first::<ShortURL>(conn);

  match item {
    Ok(expanded_url) => {
      // Increment the visitation count for the fetched URL
      let visit_update = diesel::update(&expanded_url)
        .set(visits.eq(visits + 1))
        .execute(conn);

      if visit_update.is_err() {
        warn!("Updating visit count failed for expanded url: {}", expanded_url.slug);
      }

      Ok(expanded_url)
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
  use crate::db::messages::url::CreateURL;
  use urusai_lib::models::short_url::ShortURL;

  use super::*;

  fn get_connection() -> crate::db::implementation::Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = crate::db::implementation::Database::new(database_url).unwrap();

    db.pool.get().unwrap()
  }

  fn create_url(conn: &crate::db::implementation::Connection, slug: Option<String>) -> ShortURL {
      let result = crate::db::implementation::url::create(&conn, CreateURL {
        url: "example.com".to_string(),
        slug,
        user_id: None,
      });

      result.expect("Invalid url")
  }

  fn visit_count(conn: &crate::db::implementation::Connection, short_url: &ShortURL) -> i64 {
    use urusai_lib::schema::urls::dsl::*;
    use diesel::RunQueryDsl;
    use diesel::QueryDsl;
    use diesel::ExpressionMethods;

    urls
      .filter(slug.eq(&short_url.slug))
      .first::<ShortURL>(conn)
      .expect("Unable to find URL")
      .visits
  }
  #[test]
  fn success() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let url = create_url(&conn, None);

      let read_url = read(&conn, &ReadURL {
        slug: url.slug,
      }).expect("Failed to read URL");

      assert_eq!(url.url, read_url.url);
      assert_eq!(url.visits + 1, visit_count(&conn, &read_url));

      Ok(())
    });
  }

  #[test]
  fn success_with_custom_slug() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let url = create_url(&conn, Some("custom_test_slug".to_string()));

      let read_url = read(&conn, &ReadURL {
        slug: "custom_test_slug".to_string(),
      }).expect("Failed to read URL");

      assert_eq!(url.url, read_url.url);
      assert_eq!(url.slug, read_url.slug);
      assert_eq!(url.visits + 1, visit_count(&conn, &read_url));

      Ok(())
    });
  }

  #[test]
  fn fail_not_found() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = read(&conn, &ReadURL {
        slug: "custom_test_slug".to_string(),
      });

      assert!(result.is_err());
      assert_eq!(result.err(), Some(UserError::NotFound));

      Ok(())
    });
  }
}
