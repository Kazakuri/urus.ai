#![allow(clippy::option_map_unwrap_or_else, clippy::option_map_unwrap_or)]
#![warn(
  missing_copy_implementations,
  clippy::wrong_pub_self_convention,
  clippy::mut_mut,
  clippy::non_ascii_literal,
  clippy::similar_names,
  clippy::unicode_not_nfc,
  clippy::if_not_else,
  clippy::items_after_statements,
  clippy::used_underscore_binding
)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};
use dotenv::dotenv;

pub mod db;
pub mod errors;
pub mod handlers;
pub mod templates;

#[allow(missing_debug_implementations)]
pub struct State {
  db: db::Pool,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init();

  let pool = db::create_pool();
  HttpServer::new(move || {
    let state = State { db: pool.clone() };

    App::new()
      .data(state)
      .wrap(Logger::new("\"%r\" %s %b %Dms"))
      .wrap(Compress::default())
      .configure(handlers::handlers)
  })
  .bind("0.0.0.0:3000")
  .expect("Unable to bind to port")
  .run()
  .await
}
