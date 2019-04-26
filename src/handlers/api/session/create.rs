use futures::future::Future;
use actix_web::{ http, HttpRequest, HttpResponse };
use actix_web::web::{ Data, Form };
use actix_web::middleware::identity::Identity;
use askama::Template;

use urusai_lib::models::message::{ Message, MessageType };
use crate::db::messages::session::CreateSession;
use crate::errors::UserError;
use crate::templates::Login;
use crate::State;

/// Attempts to log the user into the requested account.
///
/// Redirects back to the homepage on success, or renders the login page with an error if it fails.
pub fn create(id: Identity, form: Form<CreateSession>, req: HttpRequest) -> impl Future<Item=HttpResponse, Error=UserError> {
  let state: Data<State> = req.app_data::<State>()
    .expect("Unabled to fetch application state");
  let db = state.db.clone();

  db.send(form.into_inner())
    .timeout(std::time::Duration::new(5, 0))
    .from_err()
    .and_then(move |res| {
      match res {
        Ok(user) => {
          let token = user.id.to_string();
          id.remember(token);

          Ok(HttpResponse::SeeOther()
            .header(http::header::LOCATION, "/")
            .finish())
        },
        Err(e) => {
          Ok(HttpResponse::Ok().content_type("text/html").body(Login {
            user: &None,
            message: Some(&Message {
              message_type: MessageType::Error,
              message: &e.to_string()
            }),
          }.render().expect("Unable to render login page")))
        }
      }
    })
}

#[cfg(test)]
mod test {
  use actix::prelude::SyncArbiter;
  use actix_web::test;
  use actix_web::http::StatusCode;

  use super::*;

  use crate::db::{ DbExecutor, mock };

  #[test]
  fn success() {
    let _ = actix::System::new("urusai_test");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateSession {
        display_name: "test_account_1".to_string(),
        password: "password".to_string()
      })
      .execute(&create)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
  }

  #[test]
  fn fail_wrong_password() {
    let _ = actix::System::new("urusai_test");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateSession {
        display_name: "test_account_1".to_string(),
        password: "wrong_password".to_string()
      })
      .execute(&create)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::OK);

    match response.body() {
      actix_web::Body::Binary(b) => {
        match b {
          actix_web::Binary::Bytes(bytes) => {
            let body = std::str::from_utf8(bytes).expect("Unable to read body");
            assert!(body.contains(&UserError::LoginError.to_string()));
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

    let result = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/json")
      .set_payload(CreateSession {
        display_name: "test_account_1".to_string(),
        password: "wrong_password".to_string()
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
