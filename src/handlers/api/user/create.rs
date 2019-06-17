use actix_web::web::Form;
use actix_web::{http, HttpRequest, HttpResponse};
use askama::Template;
use futures::future::Future;

use crate::db::messages::user::CreateUser;
use crate::errors::UserError;
use crate::templates::Register;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

#[cfg(not(test))]
#[cfg(feature = "mq")]
use crate::JobQueue;

/// Tries to create a new account for the requested user.
///
/// Renders the register page with an error message if it fails.
pub fn create(
    form: Form<CreateUser>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    let state: &State = req
        .app_data::<State>()
        .expect("Unable to fetch application state");
    let db = state.db.clone();

    #[cfg(not(test))]
    #[cfg(feature = "mq")]
    let jobs = JobQueue::clone(&state.jobs);

    db.send(form.into_inner())
        .timeout(std::time::Duration::new(5, 0))
        .from_err()
        .and_then(move |res| {
            match res {
                Ok((user, token)) => {
                    #[cfg(not(test))]
                    #[cfg(feature = "mq")]
                    mq::send_activation_email(&user, &token, &jobs);

                    // TODO: Should provide a message stating that registration was successful.
                    Ok(HttpResponse::SeeOther()
                        .header(http::header::LOCATION, "/")
                        .finish())
                }
                Err(e) => Ok(HttpResponse::Ok().content_type("text/html").body(
                    Register {
                        user: &None,
                        message: Some(&Message {
                            message_type: MessageType::Error,
                            message: &e.to_string(),
                        }),
                    }
                    .render()
                    .expect("Unable to render register page"),
                )),
            }
        })
}

#[cfg(not(test))]
#[cfg(feature = "mq")]
/// Utility functions that use the message queue
mod mq {
    use faktory::{Job, Producer};
    use std::net::TcpStream;
    use std::sync::{Arc, Mutex};
    use urusai_lib::models::user::User;
    use urusai_lib::models::user_token::UserToken;

    /// Queues up a "send_activation_email" job for the provided user.
    pub fn send_activation_email(
        user: &User,
        token: &UserToken,
        jobs: &Arc<Mutex<Producer<TcpStream>>>,
    ) {
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
                    error!(
                        "Could not queue job send_activation_email for user {}",
                        user.id
                    );
                }
            }
            Err(_) => {
                error!("Could not lock job queue in send_activation_email");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use actix::prelude::SyncArbiter;
    use actix_web::http::{Cookie, StatusCode};
    use actix_web::{test, web, App};

    use super::*;

    use crate::db::{mock, DbExecutor};

    #[test]
    fn success() {
        let _sys = actix_rt::System::new("urusai_test");

        let mut app = test::init_service(
            App::new()
                .data(State {
                    db: SyncArbiter::start(1, move || {
                        DbExecutor(mock().expect("Failed to get DB instance"))
                    }),
                })
                .service(web::resource("/").to_async(create)),
        );

        let request = test::TestRequest::with_uri("/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .set_payload(CreateUser {
                display_name: "test_user".to_string(),
                email: "test@user.com".to_string(),
                password: "S3curePassw0rd!".to_string(),
            })
            .to_request();

        let response = test::call_service(&mut app, request);

        let headers = response.headers();

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert!(headers.contains_key(http::header::LOCATION));
        assert_eq!(headers.get(http::header::LOCATION).unwrap(), "/");
    }

    #[test]
    fn fail_handleable_error() {
        let _sys = actix_rt::System::new("urusai_test");

        let mut app = test::init_service(
            App::new()
                .data(State {
                    db: SyncArbiter::start(1, move || {
                        DbExecutor(mock().expect("Failed to get DB instance"))
                    }),
                })
                .service(web::resource("/").to_async(create)),
        );

        let request = test::TestRequest::with_uri("/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .set_payload(CreateUser {
                display_name: "existing_user".to_string(),
                email: "test@user.com".to_string(),
                password: "S3curePassw0rd!".to_string(),
            })
            .to_request();

        let mut response = test::call_service(&mut app, request);

        assert_eq!(response.status(), StatusCode::OK);

        match response.take_body() {
            actix_web::dev::ResponseBody::Body(b) => {
                match b {
                    actix_web::dev::Body::Bytes(bytes) => {
                        let body = std::str::from_utf8(&bytes).expect("Unable to read body");
                        assert!(body.contains(
                            &UserError::DuplicateValue {
                                field: "Display Name".to_string()
                            }
                            .to_string()
                        ));
                    }
                    _ => panic!("Unexpected binary!"),
                };
            }
            _ => panic!("Unexpected body!"),
        };
    }

    #[test]
    fn fail_missing_payload() {
        let _sys = actix_rt::System::new("urusai_test");

        let mut app = test::init_service(
            App::new()
                .data(State {
                    db: SyncArbiter::start(1, move || {
                        DbExecutor(mock().expect("Failed to get DB instance"))
                    }),
                })
                .service(web::resource("/").to_async(create)),
        );

        let request = test::TestRequest::with_uri("/").to_request();

        let response = test::call_service(&mut app, request);

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn fail_wrong_header() {
        let _sys = actix_rt::System::new("urusai_test");

        let mut app = test::init_service(
            App::new()
                .data(State {
                    db: SyncArbiter::start(1, move || {
                        DbExecutor(mock().expect("Failed to get DB instance"))
                    }),
                })
                .service(web::resource("/").to_async(create)),
        );

        let request = test::TestRequest::with_uri("/")
            .header("Content-Type", "application/json")
            .set_payload(CreateUser {
                display_name: "test_user".to_string(),
                email: "test@user.com".to_string(),
                password: "S3curePassw0rd!".to_string(),
            })
            .to_request();

        let response = test::call_service(&mut app, request);

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
