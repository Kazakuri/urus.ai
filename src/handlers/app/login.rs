use futures::future::*;
use actix_web::{ HttpRequest, HttpResponse, AsyncResponder };
use actix_web::middleware::identity::RequestIdentity;
use askama::Template;

use crate::State;
use crate::errors::UserError;
use crate::templates::Login;

/// Creates an instance of the login page, redirecting to home instead if a user is logged in.
pub fn login(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  if req.identity().is_some() {
    return ok::<HttpResponse, UserError>(HttpResponse::SeeOther()
      .header("Location", "/")
      .finish()).responder()
  }

  ok::<HttpResponse, UserError>(HttpResponse::Ok().content_type("text/html").body(Login {
    user: &None,
    message: None,
  }.render().expect("Unable to render login page"))).responder()
}

// TODO: Test
