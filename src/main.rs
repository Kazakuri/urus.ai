#![deny(clippy::all)]
#![deny(clippy::restriction)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::string_add)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::implicit_return)]
#![allow(clippy::wildcard_enum_match_arm)]
#![warn(missing_docs)]

//! Urusai code

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

use actix::prelude::*;
use actix_web::{ HttpServer, App };
use actix_web::middleware::{ Logger, Compress };
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};

#[cfg(not(test))]
#[cfg(feature = "mq")]
use faktory::Producer;

#[cfg(not(test))]
#[cfg(feature = "mq")]
use std::sync::Arc;

#[cfg(not(test))]
#[cfg(feature = "mq")]
use std::sync::Mutex;

#[cfg(not(test))]
#[cfg(feature = "mq")]
use std::net::TcpStream;

use std::env;
use dotenv::dotenv;

/// Application routes and handling.
pub mod handlers;

/// Wrapper around a connection to a persistent storage device.
pub mod db;

/// Common enumerations for errors returned throughout the application.
pub mod errors;

/// Askama templates for compiled, parameterized application views.
pub mod templates;

/// Utility functions that get called often within other modules.
pub mod utils;

use crate::db::DbExecutor;


/// An address to a `DbExecutor` actor.
pub type Database = Addr<DbExecutor>;

/// A thread-safe pointer to a Faktory job queue.
#[cfg(not(test))]
#[cfg(feature = "mq")]
pub type JobQueue = Arc<Mutex<Producer<TcpStream>>>;

/// Provides application-level state available to HTTP request handlers.
pub struct State {
  /// A connection to an actor representing a data repository.
  /// This can be a database, hard-coded values, or whatever depending on the `DataRepository` used.
  db: Database,

  #[cfg(not(test))]
  #[cfg(feature = "mq")]
  /// A connection to a Faktory job queue.
  jobs: JobQueue,
}

/// Sets up and starts the web server with all the state, middleware, and routing attached.
fn main() {
  dotenv().ok();

  std::env::set_var("RUST_LOG", "urusai,actix_web=info");
  env_logger::init();

  let sys = actix_rt::System::new("urusai");

  // Ensure that the environment variables we use are all set
  // These should all be set from .env, but
  env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  env::var("DOMAIN").expect("DOMAIN must be set");
  env::var("SECRET").expect("SECRET must be set");
  env::var("MAILER_FROM_ADDRESS").expect("MAILER_FROM_ADDRESS must be set");
  env::var("MAILER_MAIL_SERVER").expect("MAILER_MAIL_SERVER must be set");
  env::var("MAILER_USERNAME").expect("MAILER_USERNAME must be set");
  env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD must be set");
  env::var("FAKTORY_PROVIDER").expect("FAKTORY_PROVIDER must be set");
  env::var("FAKTORY_URL").expect("FAKTORY_URL must be set");

  let secret = env::var("SECRET").expect("SECRET must be set");
  let _domain = env::var("DOMAIN").expect("DOMAIN must be set");

  let addr = SyncArbiter::start(3, move || {
    DbExecutor(db::database().expect("Failed to connect to database"))
  });

  #[cfg(not(test))]
  #[cfg(feature = "mq")]
  let producer = Arc::new(
    Mutex::new(
      Producer::connect(None).expect("Failed to connect to job queue")
    )
  );

  HttpServer::new(move || {
    App::new()
      .data(State {
        db: addr.clone(),
        #[cfg(not(test))]
        #[cfg(feature = "mq")]
        jobs: JobQueue::clone(&producer),
      })
      .wrap(Logger::new("\"%r\" %s %b %Dms"))
      .wrap(Compress::default())
      .wrap(IdentityService::new(
        CookieIdentityPolicy::new(secret.as_bytes())
          .name("auth-cookie")
          .path("/")
          //.domain(domain)
          .max_age(60 * 60 * 24)
          .secure(false))
      )
      .configure(handlers::handlers)
  })
    .bind("127.0.0.1:3000")
    .expect("Unable to bind to port")
    .start();

  let _ = sys.run();
}

// TODO: Test
// - Routes
// - Template <a> tags


/* TODO: Async Actors can't be tested. https://github.com/actix/actix-web/issues/897

failures:
    handlers::api::session::create::test::fail_wrong_password
    handlers::api::session::create::test::success
    handlers::api::url::create::test::fail_handleable_error
    handlers::api::url::create::test::success_no_slug
    handlers::api::url::create::test::success_with_slug
    handlers::api::url::read::test::success
    handlers::api::user::create::test::fail_handleable_error
    handlers::api::user::create::test::success
    utils::load_user::test::success_with_user
*/
