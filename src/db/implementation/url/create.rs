use actix::Message;
use uuid::Uuid;
use lazy_static::lazy_static;
use regex::Regex;

use crate::db::messages::url::CreateURL;
use crate::db::implementation::Connection;
use urusai_lib::models::short_url::{ ShortURL, NewShortURL };
use crate::errors::UserError;

lazy_static! {
  static ref BLACKLIST: Vec<&'static str> = vec![
    "urus.ai",
    "polr.me",
    "bit.ly",
    "is.gd",
    "tiny.cc",
    "adf.ly",
    "ur1.ca",
    "goo.gl",
    "ow.ly",
    "j.mp",
    "t.co",
  ];
}

lazy_static! {
  static ref URL_REGEX: Regex = Regex::new(
    r"^(http://www\.|https://www\.|http://|https://)?[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,5}(:[0-9]{1,5})?(/.*)?$"
  ).expect("Unable to compile regex");
}

/// Creates a new association to a url from a `CreateURL` message within the `Connection`, returning the created `ShortURL` association.
pub fn create(conn: &Connection, msg: CreateURL) -> <CreateURL as Message>::Result {
  use urusai_lib::schema::urls::dsl::*;
  use diesel::RunQueryDsl;
  use diesel::QueryDsl;
  use diesel::ExpressionMethods;
  use diesel::result::{ Error, DatabaseErrorKind };

  let uuid = Uuid::new_v4();

  // We can associate a ShortURL with a certain user, giving them access to view their previously created links
  let optional_user_id = match msg.user_id {
    Some(optional_user_id) => Some(Uuid::parse_str(&optional_user_id).expect("Invalid UUID supplied")),
    None => None,
  };

  let current_url = &msg.url;

  if BLACKLIST.iter().any(|u| current_url.contains(u)) {
    return Err(UserError::LinkAlreadyShortened);
  }

  if !URL_REGEX.is_match(current_url) {
    return Err(UserError::InvalidLink);
  }

  // Providing a URL slug is optional, so generate our own URL-friendly ID if they didn't provide one
  // At 10 IDs per hour, it'll take ~27 years to have a 1% probability of a collision for an 8 character nanoid
  let url_slug = match msg.slug {
    Some(url_slug) => if !url_slug.is_empty() {
      url_slug
    } else {
      nanoid::generate(8).to_string()
    },
    None => nanoid::generate(8).to_string()
  };

  // Validate that our slug is URL-friendly
  if !url_slug.chars().all(|c| nanoid::alphabet::SAFE.iter().any(|x| *x == c)) {
    return Err(UserError::InvalidCharactersInURL);
  }

  let new_url = NewShortURL {
    id: &uuid,
    user_id: optional_user_id,
    slug: &url_slug,
    url: &msg.url,
  };

  let result = diesel::insert_into(urls)
    .values(&new_url)
    .execute(conn);

  match result {
    Err(e) => match e {
      Error::DatabaseError(kind, info) => match kind {
        DatabaseErrorKind::UniqueViolation => Err(UserError::DuplicateValue {
          field: info.constraint_name().expect("Missing column for violation").to_string()
        }),
        DatabaseErrorKind::ForeignKeyViolation => Err(UserError::UnknownValue {
          field: info.constraint_name().expect("Missing column for violation").to_string()
        }),
        _ => Err(UserError::InternalError),
      },
      _ => Err(UserError::InternalError),
    },
    Ok(_) => {
      let item = urls
        .filter(id.eq(&uuid))
        .first::<ShortURL>(conn)
        .expect("Error loading url");

      Ok(item)
    }
  }
}

#[cfg(test)]
mod tests {
  use diesel::result::Error;
  use diesel::Connection;
  use std::env;
  use dotenv::dotenv;
  use crate::db::messages::user::{ CreateUser, VerifyUser };
  use urusai_lib::models::user::User;
  use urusai_lib::models::user_token::UserToken;

  use super::*;

  fn get_connection() -> crate::db::implementation::Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = crate::db::implementation::Database::new(database_url).unwrap();

    db.pool.get().unwrap()
  }

  fn create_user(conn: &crate::db::implementation::Connection) -> (User, UserToken) {
    let result = crate::db::implementation::user::create(&conn, CreateUser {
      display_name: "test_user".to_string(),
      email: "test@user.com".to_string(),
      password: "S3curePassw0rd!".to_string(),
    });

    result.expect("Invalid user")
  }

  #[test]
  fn success_anonymous() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = create(&conn, CreateURL {
        url: "http://example.com".to_string(),
        slug: None,
        user_id: None,
      });

      let url = result.expect("Invalid ShortURL");

      assert_eq!(url.url, "http://example.com");
      assert_eq!(url.slug.len(), 8);
      assert_eq!(url.user_id, None);

      Ok(())
    });
  }

  #[test]
  fn success_anonymous_slug() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = create(&conn, CreateURL {
        url: "http://example.com".to_string(),
        slug: Some("test_slug".to_string()),
        user_id: None,
      });

      let url = result.expect("Invalid ShortURL");

      assert_eq!(url.url, "http://example.com");
      assert_eq!(url.slug, "test_slug");
      assert_eq!(url.user_id, None);

      Ok(())
    });
  }

  #[test]
  fn success_user() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, _) = create_user(&conn);

      let result = create(&conn, CreateURL {
        url: "http://example.com".to_string(),
        slug: None,
        user_id: Some(user.id.to_string()),
      });

      let url = result.expect("Invalid ShortURL");

      assert_eq!(url.url, "http://example.com");
      assert_eq!(url.slug.len(), 8);
      assert_eq!(url.user_id, Some(user.id));

      Ok(())
    });
  }

  #[test]
  fn success_user_slug() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let (user, _) = create_user(&conn);

      let result = create(&conn, CreateURL {
        url: "http://example.com".to_string(),
        slug: Some("test_slug".to_string()),
        user_id: Some(user.id.to_string()),
      });

      let url = result.expect("Invalid ShortURL");

      assert_eq!(url.url, "http://example.com");
      assert_eq!(url.slug, "test_slug");
      assert_eq!(url.user_id, Some(user.id));

      Ok(())
    });
  }

  #[test]
  fn success_thousand() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      for _ in 1..=1000 {
        let result = create(&conn, CreateURL {
          url: "http://example.com".to_string(),
          slug: None,
          user_id: None,
        });

        let url = result.expect("Invalid ShortURL");

        assert_eq!(url.url, "http://example.com");
        assert_eq!(url.slug.len(), 8);
        assert_eq!(url.user_id, None);
      }

      Ok(())
    });
  }

  #[test]
  fn fail_duplicate_slug() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = create(&conn, CreateURL {
        url: "http://example.com".to_string(),
        slug: Some("fail".to_string()),
        user_id: None,
      });

      let result = create(&conn, CreateURL {
        url: "http://example.com".to_string(),
        slug: Some("fail".to_string()),
        user_id: None,
      });

      assert_eq!(result.err(), Some(UserError::DuplicateValue { field: "Short URL".to_string() }));

      Ok(())
    });
  }

  #[test]
  fn fail_already_shortened_url() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = create(&conn, CreateURL {
        url: "http://bit.ly".to_string(),
        slug: None,
        user_id: None,
      });

      assert_eq!(result.err(), Some(UserError::LinkAlreadyShortened));

      Ok(())
    });
  }

  #[test]
  fn fail_not_a_url() {
    let conn = get_connection();

    conn.test_transaction::<_, Error, _>(|| {
      let result = create(&conn, CreateURL {
        url: "not_a_url".to_string(),
        slug: None,
        user_id: None,
      });

      assert_eq!(result.err(), Some(UserError::InvalidLink));

      Ok(())
    });
  }
}
