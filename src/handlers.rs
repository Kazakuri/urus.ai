pub mod api;

pub mod app;

use actix_files::Files;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn handlers(app: &mut ServiceConfig) {
  debug(app)
    .service(
      web::resource("/")
        .route(web::get().to(app::index))
        .route(web::post().to(api::url::create)),
    )
    .service(
      web::resource("/login")
        .route(web::get().to(app::login))
        .route(web::post().to(api::session::create)),
    )
    .service(web::resource("/logout").route(web::get().to(api::session::delete)))
    .service(
      web::resource("/register")
        .route(web::get().to(app::register))
        .route(web::post().to(api::user::create)),
    )
    .service(web::resource("/profile").route(web::get().to(app::profile::urls)))
    .service(web::resource("/profile/urls").route(web::get().to(app::profile::urls)))
    .service(web::resource("/profile/account").route(web::get().to(app::profile::account)))
    .service(web::resource("/verify/{user_id}/{id}").route(web::get().to(api::user::verify)))
    .service(web::resource("/account/password").route(web::post().to(api::user::password_change)))
    .service(web::resource("/{slug}").route(web::get().to(api::url::read)));
}

#[cfg(debug_assertions)]
fn debug(app: &mut ServiceConfig) -> &mut ServiceConfig {
  app.service(Files::new("/res", "./public"))
}

#[cfg(not(debug_assertions))]
fn debug(app: &mut ServiceConfig) -> &mut ServiceConfig {
  app
}
