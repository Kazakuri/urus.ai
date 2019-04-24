/// Handlers for API routes.
pub mod api;

/// Handlers for application views.
pub mod app;

use actix_web::{ App, http, fs };

use crate::errors::UserError;
use crate::State;

/// Takes in an Actix App and returns it with our URL handlers appended.
pub fn handlers(app: App<State>) -> App<State> {
  debug(app)
    .resource("/", |r| {
      r.method(http::Method::GET).a(app::index);
      r.method(http::Method::POST).a(api::url::create);
    })
    .resource("/login", |r| {
      r.method(http::Method::GET).a(app::login);
      r.method(http::Method::POST).a(api::session::create);
    })
    .resource("/logout", |r| {
      r.method(http::Method::GET).f(api::session::delete);
    })
    .resource("/register", |r| {
      r.method(http::Method::GET).a(app::register);
      r.method(http::Method::POST).a(api::user::create);
    })
    .resource("/profile", |r| {
      r.method(http::Method::GET).a(app::profile::urls);
    })
    .resource("/profile/urls", |r| {
      r.method(http::Method::GET).a(app::profile::urls);
    })
    .resource("/profile/account", |r| {
      r.method(http::Method::GET).a(app::profile::account);
    })
    .resource("/verify/{id}", |r| {
      r.method(http::Method::GET).a(api::user::verify);
    })
    .resource("/{slug}", |r| {
      r.method(http::Method::GET).a(api::url::read);
    })
}

#[cfg(debug_assertions)]
/// When building in debug mode, adds handlers for static files.
fn debug(app: App<State>) -> App<State> {
  app
    .handler("/res", fs::StaticFiles::new("./public").expect("Unable to load /public directory"))
    .resource("/favicon.ico", |r| {
      r.method(http::Method::GET).f(|_| file("public/favicon.ico"))
    })
    .resource("/robots.txt", |r| {
      r.method(http::Method::GET).f(|_| file("public/robots.txt"));
    })
    .resource("/sitemap.xml", |r| {
      r.method(http::Method::GET).f(|_| file("public/sitemap.xml"));
    })
}

#[cfg(not(debug_assertions))]
/// When building in release mode, let nginx handle static files.
fn debug(app: App<State>) -> App<State> {
  app
}

#[cfg(debug_assertions)]
/// Return the contents of a NamedFile for a handler.
fn file(path: &str) -> Result<fs::NamedFile, UserError> {
  Ok(fs::NamedFile::open(path)?)
}
