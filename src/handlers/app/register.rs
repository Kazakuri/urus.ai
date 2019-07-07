use actix_identity::Identity;
use actix_web::HttpResponse;
use askama::Template;
use futures::future::*;

use crate::errors::UserError;
use crate::templates::Register;

/// Creates an instance of the register page, redirecting to home instead if a user is logged in.
pub fn register(id: Identity) -> Box<Future<Item = HttpResponse, Error = UserError>> {
    if id.identity().is_some() {
        return Box::new(ok::<HttpResponse, UserError>(
            HttpResponse::SeeOther().header("Location", "/").finish(),
        ));
    }

    Box::new(ok::<HttpResponse, UserError>(
        HttpResponse::Ok().content_type("text/html").body(
            Register {
                user: &None,
                message: None,
            }
            .render()
            .expect("Unable to render register page"),
        ),
    ))
}

// TODO: Test
