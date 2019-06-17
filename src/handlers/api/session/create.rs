use actix_identity::Identity;
use actix_web::web::Form;
use actix_web::{http, HttpRequest, HttpResponse};
use askama::Template;
use futures::future::Future;

use crate::db::messages::session::CreateSession;
use crate::errors::UserError;
use crate::templates::Login;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

/// Attempts to log the user into the requested account.
///
/// Redirects back to the homepage on success, or renders the login page with an error if it fails.
pub fn create(
    id: Identity,
    form: Form<CreateSession>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    let state: &State = req
        .app_data::<State>()
        .expect("Unable to fetch application state");
    let db = state.db.clone();

    db.send(form.into_inner())
        .timeout(std::time::Duration::new(5, 0))
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let token = user.id.to_string();
                id.remember(token);

                Ok(HttpResponse::SeeOther()
                    .header(http::header::LOCATION, "/")
                    .finish())
            }
            Err(e) => Ok(HttpResponse::Ok().content_type("text/html").body(
                Login {
                    user: &None,
                    message: Some(&Message {
                        message_type: MessageType::Error,
                        message: &e.to_string(),
                    }),
                }
                .render()
                .expect("Unable to render login page"),
            )),
        })
}

#[cfg(test)]
mod test {
    use actix::prelude::SyncArbiter;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use futures::future::{lazy, ok};

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
            .set_payload(CreateSession {
                display_name: "test_account_1".to_string(),
                password: "password".to_string(),
            })
            .to_request();

        let response = test::call_service(&mut app, request);

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
    }

    #[test]
    fn fail_wrong_password() {
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
            .set_payload(CreateSession {
                display_name: "test_account_1".to_string(),
                password: "wrong_password".to_string(),
            })
            .to_request();

        let mut response = test::call_service(&mut app, request);

        assert_eq!(response.status(), StatusCode::OK);

        match response.take_body() {
            actix_web::dev::ResponseBody::Body(b) => {
                match b {
                    actix_web::dev::Body::Bytes(bytes) => {
                        let body = std::str::from_utf8(&bytes).expect("Unable to read body");
                        assert!(body.contains(&UserError::LoginError.to_string()));
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

        let request = test::TestRequest::with_uri("/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .to_request();

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
            .set_payload(CreateSession {
                display_name: "test_account_1".to_string(),
                password: "wrong_password".to_string(),
            })
            .to_request();

        let response = test::call_service(&mut app, request);

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
