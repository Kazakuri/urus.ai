use futures::future::Future;
use actix_web::{ HttpRequest, HttpResponse };
use actix_web::web::Form;
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
  let state: &State = req.app_data::<State>()
    .expect("Unable to fetch application state");
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
  use actix_web::{ web, test, App };
  use actix_web::http::{ StatusCode, Cookie };

  use super::*;

  use crate::db::{ DbExecutor, mock };

  #[test]
  fn success_no_slug() {
    let sys = actix_rt::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let mut app = test::init_service(
      App::new()
      .data(State {
        db: SyncArbiter::start(1, move || {
          DbExecutor(mock().expect("Failed to get DB instance"))
        }),
      })
      .service(web::resource("/").to_async(create))
    );

    let request = test::TestRequest::with_uri("/")
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateURL {
        url: "https://example.com".to_string(),
        slug: None,
        user_id: None
      }).to_request();

    let mut response = test::call_service(&mut app, request);

    assert_eq!(response.status(), StatusCode::OK);

    match response.take_body() {
      actix_web::dev::ResponseBody::Body(b) => {
        match b {
          actix_web::dev::Body::Bytes(bytes) => {
            let body = std::str::from_utf8(&bytes).expect("Unable to read body");
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
    let sys = actix_rt::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let mut app = test::init_service(
      App::new()
      .data(State {
        db: SyncArbiter::start(1, move || {
          DbExecutor(mock().expect("Failed to get DB instance"))
        }),
      })
      .service(web::resource("/").to_async(create))
    );

    let request = test::TestRequest::with_uri("/")
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateURL {
        url: "https://example.com".to_string(),
        slug: Some("custom_example".to_string()),
        user_id: None
      }).to_request();

    let mut response = test::call_service(&mut app, request);

    assert_eq!(response.status(), StatusCode::OK);

    match response.take_body() {
      actix_web::dev::ResponseBody::Body(b) => {
        match b {
          actix_web::dev::Body::Bytes(bytes) => {
            let body = std::str::from_utf8(&bytes).expect("Unable to read body");
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
    let sys = actix_rt::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let mut app = test::init_service(
      App::new()
      .data(State {
        db: SyncArbiter::start(1, move || {
          DbExecutor(mock().expect("Failed to get DB instance"))
        }),
      })
      .service(web::resource("/").to_async(create))
    );

    let request = test::TestRequest::with_uri("/")
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateURL {
        url: "not_a_url".to_string(),
        slug: None,
        user_id: None
      }).to_request();

    let mut response = test::call_service(&mut app, request);

    assert_eq!(response.status(), StatusCode::OK);

    match response.take_body() {
      actix_web::dev::ResponseBody::Body(b) => {
        match b {
          actix_web::dev::Body::Bytes(bytes) => {
            let body = std::str::from_utf8(&bytes).expect("Unable to read body");
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
    let sys = actix_rt::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let mut app = test::init_service(
      App::new()
      .data(State {
        db: SyncArbiter::start(1, move || {
          DbExecutor(mock().expect("Failed to get DB instance"))
        }),
      })
      .service(web::resource("/").to_async(create))
    );

    let request = test::TestRequest::with_uri("/")
      .header("Content-Type", "application/x-www-form-urlencoded")
      .to_request();

    let mut response = test::call_service(&mut app, request);

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
  }

  #[test]
  fn fail_wrong_header() {
    let sys = actix_rt::System::new("urusai_test");

    let domain = env::var("DOMAIN")
      .expect("DOMAIN must be set");

    let mut app = test::init_service(
      App::new()
      .data(State {
        db: SyncArbiter::start(1, move || {
          DbExecutor(mock().expect("Failed to get DB instance"))
        }),
      })
      .service(web::resource("/").to_async(create))
    );

    let request = test::TestRequest::with_uri("/")
      .header("Content-Type", "application/json")
      .set_payload(CreateURL {
        url: "https://example.com".to_string(),
        slug: None,
        user_id: None
      }).to_request();

    let mut response = test::call_service(&mut app, request);

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
  }
}
