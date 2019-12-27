use actix_identity::Identity;
use actix_web::{web::Data, HttpResponse};
use askama::Template;

use crate::errors::UserError;
use crate::templates::Index;
use crate::State;

pub async fn index(id: Identity, state: Data<State>) -> Result<HttpResponse, UserError> {
  let db = state.db.clone();

  let user = crate::utils::load_user(id.identity(), &db).await;

  Ok(
    HttpResponse::Ok().content_type("text/html").body(
      Index {
        user: &user,
        message: None,
        url: None,
      }
      .render()
      .expect("Unable to render index page"),
    ),
  )
}
