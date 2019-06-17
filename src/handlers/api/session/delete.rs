use actix_web::{ http, HttpResponse };
use actix_web::middleware::identity::Identity;

/// Forgets the current session for the user and redirects back to the homepage.
pub fn delete(id: Identity) -> HttpResponse {
  id.forget();
  HttpResponse::SeeOther()
    .header(http::header::LOCATION, "/")
    .finish()
}

#[cfg(test)]
mod test {
  use actix::prelude::SyncArbiter;
  use actix_web::{ web, test, App };
  use actix_web::http::{ StatusCode, Cookie };

  use super::*;

  use crate::db::{ DbExecutor, mock };

  #[test]
  fn success() {
    let sys = actix_rt::System::new("urusai_test");

    let mut app = test::init_service(
      App::new().service(web::resource("/").to_async(delete))
    );

    let request = test::TestRequest::with_uri("/")
      .to_request();

    let mut response = test::call_service(&mut app, request);

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
  }
}
