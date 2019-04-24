use futures::future::*;
use actix_web::{ HttpRequest, HttpResponse, AsyncResponder };
use actix_web::middleware::identity::RequestIdentity;
use askama::Template;

use crate::State;
use crate::errors::UserError;
use crate::templates::Index;

/// Creates an instance of the home page.
pub fn index(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let db = req.state().db.clone();
  let user = crate::utils::load_user(req.identity(), &db);

  ok::<HttpResponse, UserError>(HttpResponse::Ok().content_type("text/html").body(Index {
    user: &user,
    message: None,
    url: None,
  }.render().expect("Unable to render index page"))).responder()
}

// TODO: Test
