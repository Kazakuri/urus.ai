use actix_identity::Identity;
use actix_web::web::Form;
use actix_web::{web::Data, HttpResponse};
use askama::Template;

use crate::db::user::ChangeUserPassword;
use crate::errors::UserError;
use crate::templates::ProfileAccount;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

pub async fn password_change(
  id: Identity,
  form: Form<ChangeUserPassword>,
  state: Data<State>,
) -> Result<HttpResponse, UserError> {
  let db = state.db.clone();

  let user = crate::utils::load_user(id.identity(), &db).await;

  if user.is_none() {
    return Ok(HttpResponse::Forbidden().finish());
  }

  let user = user.unwrap();

  let mut form = form.into_inner();
  form.id = Some(user.id);

  let result = crate::db::user::password_change(&db, form).await;

  let error_message;

  let message = match result {
    Ok(_) => Message {
      message_type: MessageType::Notice,
      message: "Your password has been updated successfully",
    },
    Err(e) => {
      error_message = e.to_string();
      Message {
        message_type: MessageType::Error,
        message: &error_message,
      }
    }
  };

  Ok(
    HttpResponse::Ok().content_type("text/html").body(
      ProfileAccount {
        user: &Some(user),
        message: Some(&message),
      }
      .render()
      .expect("Unable to render profile account page"),
    ),
  )
}
