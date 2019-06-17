use actix_web::middleware::identity::Identity;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use futures::future::*;

use crate::errors::UserError;
use crate::templates::Index;
use crate::State;

/// Creates an instance of the home page.
pub fn index(
    id: Identity,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = UserError> {
    let state: &State = req
        .app_data::<State>()
        .expect("Unable to fetch application state");
    let db = state.db.clone();
    let user = crate::utils::load_user(id.identity(), &db);

    ok::<HttpResponse, UserError>(
        HttpResponse::Ok().content_type("text/html").body(
            Index {
                user: &user,
                message: None,
                url: None,
            }
            .render()
            .expect("Unable to render index page"),
        ),
    )
}

// TODO: Test
