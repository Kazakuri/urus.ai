use actix_web::{http, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::db::user::VerifyUser;
use crate::errors::UserError;
use crate::State;

pub async fn verify(req: HttpRequest) -> Result<HttpResponse, UserError> {
  let state: &State = req.app_data::<State>().expect("Unable to fetch application state");
  let db = state.db.clone();

  if req.match_info().get("id").is_none() {
    return Ok(HttpResponse::SeeOther().header(http::header::LOCATION, "/").finish());
  }

  if req.match_info().get("user_id").is_none() {
    return Ok(HttpResponse::SeeOther().header(http::header::LOCATION, "/").finish());
  }

  let id = match &req.match_info().get("id") {
    Some(id) => match Uuid::parse_str(id) {
      Ok(uuid) => uuid,
      Err(_) => Uuid::nil(),
    },
    None => Uuid::nil(),
  };

  let user_id = match &req.match_info().get("user_id") {
    Some(id) => match Uuid::parse_str(id) {
      Ok(uuid) => uuid,
      Err(_) => Uuid::nil(),
    },
    None => Uuid::nil(),
  };

  if id.is_nil() {
    return Err(UserError::InternalError);
  }

  if user_id.is_nil() {
    return Err(UserError::InternalError);
  }

  let data = VerifyUser { id, user_id };

  let user = crate::db::user::verify(&db, data).await;

  match user {
    Ok(_) => Ok(
      HttpResponse::SeeOther()
        .header(http::header::LOCATION, "/login")
        .finish(),
    ),
    Err(_) => Ok(
      HttpResponse::SeeOther()
        .header(http::header::LOCATION, "/login")
        .finish(),
    ),
  }
}
