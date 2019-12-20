use actix_identity::Identity;
use actix_web::web::Form;
use actix_web::{http, HttpRequest, HttpResponse};
use askama::Template;

use crate::db::session::CreateSession;
use crate::errors::UserError;
use crate::templates::Login;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

pub async fn create(id: Identity, form: Form<CreateSession>, req: HttpRequest) -> Result<HttpResponse, UserError> {
  let state: &State = req.app_data::<State>().expect("Unable to fetch application state");
  let db = state.db.clone();

  let user = crate::db::session::create(&db, form.into_inner()).await;

  match user {
    Ok(user) => {
      let token = user.id.to_string();
      id.remember(token);

      Ok(HttpResponse::SeeOther().header(http::header::LOCATION, "/").finish())
    }
    Err(e) => Ok(
      HttpResponse::Ok().content_type("text/html").body(
        Login {
          user: &None,
          message: Some(&Message {
            message_type: MessageType::Error,
            message: &e.to_string(),
          }),
        }
        .render()
        .expect("Unable to render login page"),
      ),
    ),
  }
}
