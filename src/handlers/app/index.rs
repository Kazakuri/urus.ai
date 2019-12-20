use actix_identity::Identity;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;

use crate::errors::UserError;
use crate::templates::Index;
use crate::State;

pub async fn index(id: Identity, req: HttpRequest) -> Result<HttpResponse, UserError> {
  let state: &State = req.app_data::<State>().expect("Unable to fetch application state");
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
