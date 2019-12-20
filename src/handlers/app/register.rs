use actix_identity::Identity;
use actix_web::HttpResponse;
use askama::Template;

use crate::errors::UserError;
use crate::templates::Register;

pub async fn register(id: Identity) -> Result<HttpResponse, UserError> {
  if id.identity().is_some() {
    return Ok(HttpResponse::SeeOther().header("Location", "/").finish());
  }

  Ok(
    HttpResponse::Ok().content_type("text/html").body(
      Register {
        user: &None,
        message: None,
      }
      .render()
      .expect("Unable to render register page"),
    ),
  )
}
