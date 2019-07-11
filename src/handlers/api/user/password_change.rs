use actix_web::web::Form;
use actix_web::{http, HttpRequest, HttpResponse};
use askama::Template;
use futures::future::Future;
use actix_identity::Identity;

use crate::db::messages::user::ChangeUserPassword;
use crate::errors::UserError;
use crate::templates::ProfileAccount;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};


/// Tries to change the currently logged in user's password
pub fn password_change(
    id: Identity,
    form: Form<ChangeUserPassword>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    let state: &State = req
        .app_data::<State>()
        .expect("Unable to fetch application state");
    let db = state.db.clone();

    let user = crate::utils::load_user(id.identity(), &db);

    db.send(form.into_inner())
        .timeout(std::time::Duration::new(5, 0))
        .from_err()
        .and_then(move |res| {
            let mut error_message = String::new();
            let message = match res {
                Ok(_) => Message {
                    message_type: MessageType::Notice,
                    message: "Your password has been updated successfully"
                },
                Err(e) => {
                    error_message = e.to_string();
                    Message {
                        message_type: MessageType::Error,
                        message: &error_message,
                    }
                }
            };
            
            Ok(HttpResponse::Ok().content_type("text/html").body(
                ProfileAccount {
                    user: &user,
                    message: Some(&message)
                }
                .render()
                .expect("Unable to render profile account page"),
            ))
        })
}
