use actix_web::{web::Data, HttpResponse};
use askama::Template;

use crate::errors::UserError;
use crate::templates::Index;
use crate::State;

pub async fn index(_state: Data<State>) -> Result<HttpResponse, UserError> {
  Ok(
    HttpResponse::Ok().content_type("text/html").body(
      Index {
        message: None,
        url: None,
      }
      .render()
      .expect("Unable to render index page"),
    ),
  )
}
