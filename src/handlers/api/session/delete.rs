use actix_web::{ http, HttpRequest, HttpResponse };
use actix_web::middleware::identity::RequestIdentity;

use crate::State;

/// Forgets the current session for the user and redirects back to the homepage.
pub fn delete(req: &HttpRequest<State>) -> HttpResponse {
  req.forget();
  HttpResponse::SeeOther()
    .header(http::header::LOCATION, "/")
    .finish()
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
      .execute(&delete)
      .expect("HTTP request failed");

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
  }
}
