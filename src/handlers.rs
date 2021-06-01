pub mod api;

pub mod app;

use actix_web::web;
use actix_web::web::ServiceConfig;

#[cfg(debug_assertions)]
use actix_files::Files;

pub fn handlers(app: &mut ServiceConfig) {
  debug(app)
    .service(
      web::resource("/")
        .route(web::get().to(app::index))
        .route(web::post().to(api::url::create)),
    )
    .service(web::resource("/{slug}").route(web::get().to(api::url::read)));
}

#[cfg(debug_assertions)]
fn debug(app: &mut ServiceConfig) -> &mut ServiceConfig {
  app.service(Files::new("/res", "./public/res"))
}

#[cfg(not(debug_assertions))]
fn debug(app: &mut ServiceConfig) -> &mut ServiceConfig {
  app
}
