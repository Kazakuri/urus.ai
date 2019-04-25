use futures::future::*;
use actix_web::{ HttpRequest, HttpResponse, };
use actix_web::web::Data;
use actix_web::middleware::identity::Identity;
use askama::Template;

use crate::State;
use crate::errors::UserError;
use crate::templates::Index;

/// Creates an instance of the home page.
pub fn index(id: Identity, req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let state: Data<State> = req.app_data::<State>()
    .expect("Unabled to fetch application state");
  let db = state.db.clone();
  let user = crate::utils::load_user(id.identity(), &db);

  Box::new(ok::<HttpResponse, UserError>(HttpResponse::Ok().content_type("text/html").body(Index {
    user: &user,
    message: None,
    url: None,
  }.render().expect("Unable to render index page"))))
}

// TODO: Test
