use actix_identity::Identity;
use actix_web::{http, HttpRequest, HttpResponse};
use askama::Template;
use futures::future::Future;
use futures::future::*;

use crate::db::messages::url::ReadURL;
use crate::errors::UserError;
use crate::templates::Index;
use crate::State;
use urusai_lib::models::message::{Message, MessageType};

/// Tries to redirect the user to the full url of the requested slug.
///
/// Renders the index page with an error message if it fails.
pub fn read(id: Identity, req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = UserError>> {
    let state: &State = req
        .app_data::<State>()
        .expect("Unable to fetch application state");
    let db = state.db.clone();

    let user = crate::utils::load_user(id.identity(), &db);

    if req.match_info().get("slug").is_none() {
        return Box::new(ok::<HttpResponse, UserError>(
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
        ));
    }

    let data = ReadURL {
        slug: req
            .match_info()
            .get("slug")
            .expect("Unable to get slug")
            .to_string(),
    };

    Box::new(
        db.send(data)
            .timeout(std::time::Duration::new(5, 0))
            .from_err()
            .and_then(move |res| match res {
                Ok(url) => Ok(HttpResponse::SeeOther()
                    .header(http::header::LOCATION, url.url)
                    .finish()),
                Err(_e) => Ok(HttpResponse::NotFound().content_type("text/html").body(
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
                )),
            }),
    )
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
                .service(web::resource("/{slug}").to_async(read)),
        );

        let request = test::TestRequest::with_uri("/example").to_request();

        let response = test::call_service(&mut app, request);

        let headers = response.headers();

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert!(headers.contains_key(http::header::LOCATION));
        assert_eq!(
            headers.get(http::header::LOCATION).unwrap(),
            "https://example.com"
        );
    }

    #[test]
    fn fail_not_found() {
        let _sys = actix_rt::System::new("urusai_test");

        let mut app = test::init_service(
            App::new()
                .data(State {
                    db: SyncArbiter::start(1, move || {
                        DbExecutor(mock().expect("Failed to get DB instance"))
                    }),
                })
                .service(web::resource("/").to_async(read)),
        );

        let request = test::TestRequest::with_uri("/")
            .param("slug", "unknown")
            .to_request();

        let response = test::call_service(&mut app, request);

        let headers = response.headers();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
