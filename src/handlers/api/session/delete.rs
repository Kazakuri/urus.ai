use actix_identity::Identity;
use actix_web::{http, HttpResponse};

pub fn delete(id: Identity) -> HttpResponse {
  id.forget();
  HttpResponse::SeeOther().header(http::header::LOCATION, "/").finish()
}
