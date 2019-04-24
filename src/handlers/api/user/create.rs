use futures::future::Future;
use actix_web::{ http, HttpRequest, HttpResponse, AsyncResponder, HttpMessage };
use askama::Template;

use urusai_lib::models::message::{ Message, MessageType };
use crate::db::messages::user::CreateUser;
use crate::State;
use crate::errors::UserError;
use crate::templates::Register;

#[cfg(not(test))]
#[cfg(feature = "mq")]
use crate::JobQueue;

/// Tries to create a new account for the requested user.
///
/// Renders the register page with an error message if it fails.
pub fn create(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let db = req.state().db.clone();

  #[cfg(not(test))]
  #[cfg(feature = "mq")]
  let jobs = JobQueue::clone(&req.state().jobs);

  req.urlencoded::<CreateUser>()
    .from_err()
    .and_then(move |data: CreateUser| {
      db.send(data)
        .timeout(std::time::Duration::new(5, 0))
        .from_err()
    })
    .and_then(move |res| {
      match res {
        Ok(_user) => {
          #[cfg(not(test))]
          #[cfg(feature = "mq")]
          mq::send_activation_email(&_user, &jobs);

          // TODO: Should provide a message stating that registration was successful.
          Ok(HttpResponse::SeeOther()
            .header(http::header::LOCATION, "/")
            .finish())
        },
        Err(e) => {
          Ok(HttpResponse::Ok().content_type("text/html").body(Register {
            user: &None,
            message: Some(&Message {
              message_type: MessageType::Error,
              message: &e.to_string()
            }),
          }.render().expect("Unable to render register page")))
        }
      }
    })
    .responder()
}

#[cfg(not(test))]
#[cfg(feature = "mq")]
/// Utility functions that use the message queue
mod mq {
  use std::net::TcpStream;
  use faktory::{ Job, Producer };
  use std::sync::{ Arc, Mutex };
  use urusai_lib::models::user::User;

  /// Queues up a "send_activation_email" job for the provided user.
  pub fn send_activation_email(user: &User, jobs: &Arc<Mutex<Producer<TcpStream>>>) {
    let job = Job::new("send_activation_email", vec![ serde_json::to_string(&user).expect("Could not serialize User object") ]);

    match jobs.lock() {
      Ok(mut jobs) => if jobs.enqueue(job).is_err() {
        error!("Could not queue job send_activation_email for user {}", user.id);
      },
      Err(_) => {
        error!("Could not lock job queue in send_activation_email");
      }
    }
  }
}

#[cfg(test)]
mod test {
  use actix::prelude::SyncArbiter;
  use actix_web::test;
  use actix_web::http::{ StatusCode, Cookie };

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
      .set_payload(CreateUser {
        display_name: "test_user".to_string(),
        email: "test@user.com".to_string(),
        password: "S3curePassw0rd!".to_string(),
      })
      .execute(&create)
      .expect("HTTP request failed");

    let headers = response.headers();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert!(headers.contains_key(http::header::LOCATION));
    assert_eq!(headers[http::header::LOCATION], "/");
  }

  #[test]
  fn fail_handleable_error() {
    let _ = actix::System::new("urusai_test");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .header("Content-Type", "application/x-www-form-urlencoded")
      .set_payload(CreateUser {
        display_name: "existing_user".to_string(),
        email: "test@user.com".to_string(),
        password: "S3curePassw0rd!".to_string(),
      })
      .execute(&create)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::OK);

    match response.body() {
      actix_web::Body::Binary(b) => {
        match b {
          actix_web::Binary::Bytes(bytes) => {
            let body = std::str::from_utf8(bytes).expect("Unable to read body");
            assert!(body.contains(&UserError::DuplicateValue {
              field: "Display Name".to_string()
            }.to_string()));
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
      .set_payload(CreateUser {
        display_name: "test_user".to_string(),
        email: "test@user.com".to_string(),
        password: "S3curePassw0rd!".to_string(),
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

