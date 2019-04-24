use futures::future::*;
use actix_web::{ HttpRequest, HttpResponse, AsyncResponder };
use actix_web::middleware::identity::RequestIdentity;
use askama::Template;

use crate::State;
use crate::errors::UserError;
use crate::templates::Register;

/// Creates an instance of the register page, redirecting to home instead if a user is logged in.
pub fn register(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  if req.identity().is_some() {
    return ok::<HttpResponse, UserError>(HttpResponse::SeeOther()
      .header("Location", "/")
      .finish()).responder()
  }

  ok::<HttpResponse, UserError>(HttpResponse::Ok().content_type("text/html").body(Register {
    user: &None,
    message: None,
  }.render().expect("Unable to render register page"))).responder()
}

// TODO: Test
