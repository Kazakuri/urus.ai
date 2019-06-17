use futures::future::*;
use actix_web::{ HttpRequest, HttpResponse, };
use actix_web::middleware::identity::Identity;
use askama::Template;
use uuid::Uuid;

use crate::db::messages::user::ReadUser;
use crate::State;
use crate::errors::UserError;
use crate::templates::ProfileAccount;

/// Creates an instance of the user's profile page, redirecting to home instead if the user is not logged in.
pub fn account(id: Identity, req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=UserError>> {
  let state: &State = req.app_data::<State>()
    .expect("Unable to fetch application state");

  if let Some(id) = id.identity() {
    let db = state.db.clone();
    let user_info = ReadUser {
      id: Uuid::parse_str(&id).expect("Unable to parse UUID")
    };

    return Box::new(db.send(user_info)
      .timeout(std::time::Duration::new(5, 0))
      .from_err()
      .and_then(move |res| {
        match res {
          Ok(user) => {
            Ok(HttpResponse::Ok().content_type("text/html").body(ProfileAccount {
              user: &Some(user),
              message: None,
            }.render().expect("Unable to render profile account page")))
          },
          Err(_e) => {
            Ok(HttpResponse::SeeOther()
              .header("Location", "/")
              .finish()
            )
          }
        }
      }))
  }

  Box::new(ok::<HttpResponse, UserError>(HttpResponse::SeeOther()
    .header("Location", "/")
    .finish()))
}

// TODO: Test
