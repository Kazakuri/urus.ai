use actix_identity::Identity;
use actix_web::{http, web::Data, HttpRequest, HttpResponse};
use askama::Template;

use crate::db::url::ReadURL;
use crate::errors::UserError;
use crate::templates::Index;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

pub async fn read(id: Identity, state: Data<State>, req: HttpRequest) -> Result<HttpResponse, UserError> {
  let db = state.db.clone();

  let user = crate::utils::load_user(id.identity(), &db).await;

  if req.match_info().get("slug").is_none() {
    return Ok(
      HttpResponse::NotFound().content_type("text/html").body(
        Index {
          user: &user,
          message: Some(&Message {
            message_type: MessageType::Error,
            message: "Not Found",
          }),
          url: None,
        }
        .render()
        .expect("Unable to render index page"),
      ),
    );
  }

  let data = ReadURL {
    slug: req.match_info().get("slug").expect("Unable to get slug").to_string(),
  };

  let url = crate::db::url::read(&db, data).await;

  match url {
    Ok(url) => Ok(
      HttpResponse::SeeOther()
        .header(http::header::LOCATION, url.url)
        .finish(),
    ),
    Err(_e) => Ok(
      HttpResponse::NotFound().content_type("text/html").body(
        Index {
          user: &user,
          message: Some(&Message {
            message_type: MessageType::Error,
            message: "Not Found",
          }),
          url: None,
        }
        .render()
        .expect("Unable to render index page"),
      ),
    ),
  }
}
