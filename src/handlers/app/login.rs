use actix_identity::Identity;
use actix_web::HttpResponse;
use askama::Template;

use crate::errors::UserError;
use crate::templates::Login;

pub async fn login(id: Identity) -> Result<HttpResponse, UserError> {
  if id.identity().is_some() {
    return Ok(HttpResponse::SeeOther().header("Location", "/").finish());
  }

  Ok(
    HttpResponse::Ok().content_type("text/html").body(
      Login {
        user: &None,
        message: None,
      }
      .render()
      .expect("Unable to render login page"),
    ),
  )
}
