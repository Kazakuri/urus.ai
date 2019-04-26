use futures::future::Future;
use actix_web::{ HttpRequest, HttpResponse };
use actix_web::web::{ Data, Form };
use actix_web::middleware::identity::Identity;
use std::env;
use askama::Template;

use crate::db::messages::url::CreateURL;
use crate::State;
use crate::errors::UserError;
use crate::templates::Index;
use urusai_lib::models::message::{ Message, MessageType };

/// Tries to create a ShortURL for the provided url.
pub fn create(id: Identity, mut form: Form<CreateURL>, req: HttpRequest) -> impl Future<Item=HttpResponse, Error=UserError> {
  let state: Data<State> = req.app_data::<State>()
    .expect("Unabled to fetch application state");
  let db = state.db.clone();

  let user = crate::utils::load_user(id.identity(), &db);

  let domain = env::var("DOMAIN")
    .expect("DOMAIN must be set");
    
  if let Some(id) = id.identity() {
    form.user_id = Some(id);
  }

  db.send(form.into_inner())
    .timeout(std::time::Duration::new(5, 0))
    .from_err()
    .and_then(move |res| {
      match res {
        Ok(url) => {
            Ok(HttpResponse::Ok().content_type("text/html").body(Index {
              user: &user,
              message: None,
              url: Some(&format!("https://{}/{}", domain, url.slug)),
            }.render().expect("Unable to render index page")))
        },
        Err(e) => {
            Ok(HttpResponse::Ok().content_type("text/html").body(Index {
              user: &user,
              message: Some(&Message {
                message_type: MessageType::Error,
                message: &e.to_string()
              }),
              url: None,
            }.render().expect("Unable to render index page")))
        }
      }
    })
}

#[cfg(test)]
mod test {
  use actix::prelude::SyncArbiter;
  use actix_web::test;
  use actix_web::http::{ StatusCode, Cookie };

  use super::*;

  use crate::db::{ DbExecutor, mock };

  #[test]
  fn success_no_slug() {
    let _ = actix::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateURL {
        url: "https://example.com".to_string(),
        slug: None,
        user_id: None
      })
      .execute(&create)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::OK);

    match response.body() {
      actix_web::Body::Binary(b) => {
        match b {
          actix_web::Binary::Bytes(bytes) => {
            let body = std::str::from_utf8(bytes).expect("Unable to read body");
            assert!(body.contains(&format!("Your shortened URL has been created: https:&#x2f;&#x2f;{}&#x2f;test_slug", domain)));
          }
          _ => panic!("Unexpected binary!")
        };
      },
      _ => panic!("Unexpected body!")
    };
  }

  #[test]
  fn success_with_slug() {
    let _ = actix::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateURL {
        url: "https://example.com".to_string(),
        slug: Some("custom_example".to_string()),
        user_id: None
      })
      .execute(&create)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::OK);

    match response.body() {
      actix_web::Body::Binary(b) => {
        match b {
          actix_web::Binary::Bytes(bytes) => {
            let body = std::str::from_utf8(bytes).expect("Unable to read body");
            assert!(body.contains(&format!("Your shortened URL has been created: https:&#x2f;&#x2f;{}&#x2f;custom_example", domain)));
          }
          _ => panic!("Unexpected binary!")
        };
      },
      _ => panic!("Unexpected body!")
    };
  }

  #[test]
  fn fail_handleable_error() {
    let _ = actix::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateURL {
        url: "not_a_url".to_string(),
        slug: None,
        user_id: None
      })
      .execute(&create)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::OK);

    match response.body() {
      actix_web::Body::Binary(b) => {
        match b {
          actix_web::Binary::Bytes(bytes) => {
            let body = std::str::from_utf8(bytes).expect("Unable to read body");
            assert!(body.contains(&UserError::InvalidCharactersInURL.to_string()));
          }
          _ => panic!("Unexpected binary!")
        };
      },
      _ => panic!("Unexpected body!")
    };
  }

  #[test]
  fn fail_missing_payload() {
    let _ = actix::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let result = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .execute(&create);

    assert!(result.is_err());

    match result.err() {
      Some(e) => {
        assert_eq!(e.cause().error_response().status(), StatusCode::BAD_REQUEST);
      }
      _ => panic!("This request should not have succeeded")
    };
  }

  #[test]
  fn fail_wrong_header() {
    let _ = actix::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let result = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/json")
      .set_payload(CreateURL {
        url: "https://example.com".to_string(),
        slug: None,
        user_id: None
      })
      .execute(&create);

    assert!(result.is_err());

    match result.err() {
      Some(e) => {
        assert_eq!(e.cause().error_response().status(), StatusCode::BAD_REQUEST);
      }
      _ => panic!("This request should not have succeeded")
    };
  }
}
