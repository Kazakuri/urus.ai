/// Handlers for API routes.
pub mod api;

/// Handlers for application views.
pub mod app;

use actix_web::{ web };
use actix_web::web::ServiceConfig;
use actix_files::{ Files, NamedFile };

use crate::errors::UserError;

/// Takes in an Actix App and returns it with our URL handlers appended.
pub fn handlers(app: &mut ServiceConfig) {
  debug(app)
    .service(
      web::resource("/")
        .route(web::get().to_async(app::index))
        .route(web::post().to_async(api::url::create))
    )
    .service(
      web::resource("/login")
        .route(web::get().to_async(app::login))
        .route(web::post().to_async(api::session::create))
    )
    .service(
      web::resource("/logout")
        .route(web::get().to_async(api::session::delete))
    )
    .service(
      web::resource("/register")
        .route(web::get().to_async(app::register))
        .route(web::post().to_async(api::user::create))
    )
    .service(
      web::resource("/profile")
        .route(web::get().to_async(app::profile::urls))
    )
    .service(
      web::resource("/profile/urls")
        .route(web::get().to_async(app::profile::urls))
    )
    .service(
      web::resource("/profile/account")
        .route(web::get().to_async(app::profile::account))
    )
    .service(
      web::resource("/verify/{user_id}/{id}")
        .route(web::get().to_async(api::user::verify))
    )
    .service(
      web::resource("/{slug}")
        .route(web::get().to_async(api::url::read))
    );
}

#[cfg(debug_assertions)]
/// When building in debug mode, adds handlers for static files.
fn debug(app: &mut ServiceConfig) -> &mut ServiceConfig {
  app
    .service(Files::new("/res", "./public"))
    .service(
      web::resource("/favicon.ico")
        .route(web::get().to(|| file("public/favicon.ico")))
    )
    .service(
      web::resource("/robots.txt")
        .route(web::get().to(|| file("public/robots.txt")))
    )
    .service(
      web::resource("/sitemap.xml")
        .route(web::get().to(|| file("public/sitemap.xml")))
    )
}

#[cfg(not(debug_assertions))]
/// When building in release mode, let nginx handle static files.
fn debug(app: &mut ServiceConfig) -> &mut ServiceConfig {
  app
}

#[cfg(debug_assertions)]
/// Return the contents of a NamedFile for a handler.
fn file(path: &str) -> Result<NamedFile, UserError> {
  Ok(NamedFile::open(path)?)
}
