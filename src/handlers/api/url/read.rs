use askama::Template;
use futures::future::*;
use futures::future::Future;
use actix_web::{ http, HttpRequest, HttpResponse, AsyncResponder };
use actix_web::middleware::identity::RequestIdentity;

use urusai_lib::models::message::{ Message, MessageType };
use crate::db::messages::url::ReadURL;
use crate::State;
use crate::errors::UserError;
use crate::templates::Index;

/// Tries to redirect the user to the full url of the requested slug.
///
/// Renders the index page with an error message if it fails.
pub fn read(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let db = req.state().db.clone();

  let _session = req.clone();
  let user = crate::utils::load_user(req.identity(), &db);

  if req.match_info().get("slug").is_none() {
    return ok::<HttpResponse, UserError>(HttpResponse::NotFound().content_type("text/html").body(Index {
      user: &user,
      message: Some(&Message {
        message_type: MessageType::Error,
        message: "Not Found"
      }),
      url: None,
    }.render().expect("Unable to render index page"))).responder();
  }

  let data = ReadURL {
    slug: req.match_info().get("slug").expect("Unable to get slug").to_string()
  };

  db.send(data)
    .timeout(std::time::Duration::new(5, 0))
    .from_err()
    .and_then(move |res| {
      match res {
        Ok(url) => {
          Ok(HttpResponse::SeeOther()
            .header(http::header::LOCATION, url.url)
            .finish())
        },
        Err(_e) => Ok(HttpResponse::NotFound().content_type("text/html").body(Index {
          user: &user,
          message: Some(&Message {
            message_type: MessageType::Error,
            message: "Not Found"
          }),
          url: None,
        }.render().expect("Unable to render index page")))
      }
    })
    .responder()
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
      .param("slug", "example")
      .execute(&read)
      .expect("HTTP request failed");

    let headers = response.headers();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert!(headers.contains_key(http::header::LOCATION));
    assert_eq!(headers[http::header::LOCATION], "https://example.com");
  }

  #[test]
  fn fail_not_found() {
    let _ = actix::System::new("urusai_test");

    let response = test::TestRequest::with_state(State {
      db: SyncArbiter::start(1, move || {
        DbExecutor(mock().expect("Failed to get DB instance"))
      }),
    })
      .param("slug", "unknown")
      .execute(&read)
      .expect("HTTP request failed");

    let headers = response.headers();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
  }
}
