use futures::future::Future;
use actix_web::{ http, HttpRequest, HttpResponse, AsyncResponder };
use futures::future::{ ok, err };
use uuid::Uuid;

use crate::db::messages::user::VerifyUser;
use crate::State;
use crate::errors::UserError;

/// Verifies a user's account based on the passed in ID.
///
/// This URL should be the only time they "know" their ID.
pub fn verify(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let db = req.state().db.clone();

    if req.match_info().get("id").is_none() {
      return ok::<HttpResponse, UserError>(HttpResponse::SeeOther()
        .header(http::header::LOCATION, "/")
        .finish()).responder();
    }

  let id = match &req.match_info().get("id") {
    Some(id) => match Uuid::parse_str(id) {
      Ok(uuid) => uuid,
      Err(_) => Uuid::nil()
    },
    None => Uuid::nil()
  };

  if id.is_nil() {
    return err::<HttpResponse, UserError>(UserError::InternalError).responder();
  }

  let data = VerifyUser { id };

  db.send(data)
    .timeout(std::time::Duration::new(5, 0))
    .from_err()
    .and_then(|res| {
      match res {
        Ok(_user) => {
          Ok(HttpResponse::SeeOther()
            .header(http::header::LOCATION, "/login")
            .finish())
        },
        Err(_e) => {
          Ok(HttpResponse::SeeOther()
            .header(http::header::LOCATION, "/login")
            .finish())
        }
      }
    })
    .responder()
}

// TODO: Test
