use actix_web::web::Form;
use actix_web::{web::Data, HttpResponse};
use askama::Template;
use std::env;

use crate::db::url::CreateURL;
use crate::errors::UserError;
use crate::templates::Index;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

pub async fn create(form: Form<CreateURL>, state: Data<State>) -> Result<HttpResponse, UserError> {
  let db = state.db.clone();

  let domain = env::var("DOMAIN").expect("DOMAIN must be set");

  let url = crate::db::url::create(&db, form.into_inner()).await;

  match url {
    Ok(url) => Ok(
      HttpResponse::Ok().content_type("text/html").body(
        Index {
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
