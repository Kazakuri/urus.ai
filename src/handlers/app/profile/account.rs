use actix_identity::Identity;
use actix_web::{web::Data, HttpResponse};
use askama::Template;
use uuid::Uuid;

use crate::db::user::ReadUser;
use crate::errors::UserError;
use crate::templates::ProfileAccount;
use crate::State;

pub async fn account(id: Identity, state: Data<State>) -> Result<HttpResponse, UserError> {
  if let Some(id) = id.identity() {
    let db = state.db.clone();

    let user_info = ReadUser {
      id: Uuid::parse_str(&id).expect("Unable to parse UUID"),
    };

    let profile = crate::db::user::read(&db, user_info).await;

    return match profile {
      Ok(user) => Ok(
        HttpResponse::Ok().content_type("text/html").body(
          ProfileAccount {
            user: &Some(user),
            message: None,
          }
          .render()
          .expect("Unable to render profile account page"),
        ),
      ),
      Err(_e) => Ok(HttpResponse::SeeOther().header("Location", "/").finish()),
    };
  }

  Ok(HttpResponse::SeeOther().header("Location", "/").finish())
}
