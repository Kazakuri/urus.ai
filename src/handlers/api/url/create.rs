use actix_identity::Identity;
use actix_web::web::Form;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use std::env;

use crate::db::url::CreateURL;
use crate::errors::UserError;
use crate::templates::Index;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

pub async fn create(id: Identity, mut form: Form<CreateURL>, req: HttpRequest) -> Result<HttpResponse, UserError> {
  let state: &State = req.app_data::<State>().expect("Unable to fetch application state");
  let db = state.db.clone();

  let user = crate::utils::load_user(id.identity(), &db).await;

  let domain = env::var("DOMAIN").expect("DOMAIN must be set");

  if let Some(id) = id.identity() {
    form.user_id = Some(id);
  }

  let url = crate::db::url::create(&db, form.into_inner()).await;

  match url {
    Ok(url) => Ok(
      HttpResponse::Ok().content_type("text/html").body(
        Index {
          user: &user,
          message: None,
          url: Some(&format!("https://{}/{}", domain, url)),
        }
        .render()
        .expect("Unable to render index page"),
      ),
    ),
    Err(e) => Ok(
      HttpResponse::Ok().content_type("text/html").body(
        Index {
          user: &user,
          message: Some(&Message {
            message_type: MessageType::Error,
            message: &e.to_string(),
          }),
          url: None,
        }
        .render()
        .expect("Unable to render index page"),
      ),
    ),
  }
}
