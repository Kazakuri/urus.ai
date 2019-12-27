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

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};

#[cfg(all(not(test), feature = "mq"))]
use faktory::Producer;

#[cfg(all(not(test), feature = "mq"))]
use std::sync::Arc;

#[cfg(all(not(test), feature = "mq"))]
use std::sync::Mutex;

#[cfg(all(not(test), feature = "mq"))]
use std::net::TcpStream;

use dotenv::dotenv;
use std::env;

pub mod db;
pub mod errors;
pub mod handlers;
pub mod templates;
pub mod utils;

#[cfg(all(not(test), feature = "mq"))]
pub type JobQueue = Arc<Mutex<Producer<TcpStream>>>;

#[allow(missing_debug_implementations)]
pub struct State {
  db: db::Pool,

  #[cfg(all(not(test), feature = "mq"))]
  jobs: JobQueue,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init();

  let secret = env::var("SECRET").expect("SECRET must be set");

  let pool = db::create_pool();

  #[cfg(all(not(test), feature = "mq"))]
  let producer = Arc::new(Mutex::new(
    Producer::connect(None).expect("Failed to connect to job queue"),
  ));

  HttpServer::new(move || {
    // I should be able to combine these two to a single statement with the cfg check on `jobs`.
    // Unfortunately that seems to not work and the cfg check always passes, leading to a compilation error.
    #[cfg(all(not(test), feature = "mq"))]
    let state = State {
      db: pool.clone(),
      jobs: JobQueue::clone(&producer),
    };

    #[cfg(any(test, not(feature = "mq")))]
    let state = State {
      db: pool.clone(),
    };

    App::new()
      .data(state)
      .wrap(Logger::new("\"%r\" %s %b %Dms"))
      .wrap(Compress::default())
      .wrap(IdentityService::new(
        CookieIdentityPolicy::new(secret.as_bytes())
          .name("auth-cookie")
          .path("/")
          //.domain(domain)
          .max_age(60 * 60 * 24)
          .secure(false),
      ))
      .configure(handlers::handlers)
  })
  .bind("0.0.0.0:3000")
  .expect("Unable to bind to port")
  .run()
  .await
}
