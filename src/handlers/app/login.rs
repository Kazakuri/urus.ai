use futures::future::*;
use actix_web::{ HttpRequest, HttpResponse, };
use actix_web::web::Data;
use actix_web::middleware::identity::Identity;
use askama::Template;

use crate::State;
use crate::errors::UserError;
use crate::templates::Login;

/// Creates an instance of the login page, redirecting to home instead if a user is logged in.
pub fn login(id: Identity, req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let state: Data<State> = req.app_data::<State>()
    .expect("Unabled to fetch application state");
  if id.identity().is_some() {
    return Box::new(ok::<HttpResponse, UserError>(HttpResponse::SeeOther()
      .header("Location", "/")
      .finish()))
  }

  Box::new(ok::<HttpResponse, UserError>(HttpResponse::Ok().content_type("text/html").body(Login {
    user: &None,
    message: None,
  }.render().expect("Unable to render login page"))))
}

// TODO: Test
