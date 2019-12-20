use actix_web::web::Form;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;

use crate::db::user::CreateUser;
use crate::errors::UserError;
use crate::templates::Register;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

#[cfg(all(not(test), feature = "mq"))]
use crate::JobQueue;

pub async fn create(form: Form<CreateUser>, req: HttpRequest) -> Result<HttpResponse, UserError> {
  let state: &State = req.app_data::<State>().expect("Unable to fetch application state");
  let db = state.db.clone();

  #[cfg(all(not(test), feature = "mq"))]
  let jobs = JobQueue::clone(&state.jobs);

  let user = crate::db::user::create(&db, form.into_inner()).await;

  match user {
    // `user` and `token` are only used to feed into sending the activation email.
    // If the message queue feature is disabled a warning would be raised.
    #[allow(unused_variables)]
    Ok((user, token)) => {
      #[cfg(all(not(test), feature = "mq"))]
      mq::send_activation_email(&user, &token, &jobs);

      Ok(HttpResponse::Ok().content_type("text/html").body(
        Register {
          user: &None,
          message: Some(&Message {
            message_type: MessageType::Notice,
            message: "Registration successful. You should receive an e-mail with instructions to activate your account shortly!",
          }),
        }
        .render()
        .expect("Unable to render register page"),
      ))
    }
    Err(e) => Ok(
      HttpResponse::Ok().content_type("text/html").body(
        Register {
          user: &None,
          message: Some(&Message {
            message_type: MessageType::Error,
            message: &e.to_string(),
          }),
        }
        .render()
        .expect("Unable to render register page"),
      ),
    ),
  }
}

#[cfg(all(not(test), feature = "mq"))]
mod mq {
  use faktory::Job;

  use crate::JobQueue;

  use urusai_lib::models::user::User;
  use urusai_lib::models::user_token::UserToken;

  pub fn send_activation_email(user: &User, token: &UserToken, jobs: &JobQueue) {
    let job = Job::new(
      "send_activation_email",
      vec![
        serde_json::to_string(&user).expect("Could not serialize User object"),
        serde_json::to_string(&token).expect("Could not serialize User object"),
      ],
    );

    match jobs.lock() {
      Ok(mut jobs) => {
        if jobs.enqueue(job).is_err() {
          error!("Could not queue job send_activation_email for user {}", user.id);
        }
      }
      Err(_) => {
        error!("Could not lock job queue in send_activation_email");
      }
    }
  }
}
