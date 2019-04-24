#![deny(clippy::all)]
#![deny(clippy::restriction)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::string_add)]
#![allow(clippy::integer_arithmetic)]
#![feature(never_type)]

//! Urusai code

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

use actix::prelude::*;
use actix_web::{ server, App };

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

/// Askama templates for compiled, paramterized application views.
pub mod templates;

/// Actix middleware to be run before every request is handled.
pub mod middleware;

/// Utility functions that get called often within other modules.
pub mod utils;

use crate::db::DbExecutor;

use crate::middleware::init_middleware;

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

  let sys = actix::System::new("urusai");

  // Ensure that the environment variables we use are all set
  // These should all be set from .env, but
  env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  env::var("DOMAIN").expect("DOMAIN must be set");
  env::var("SECRET").expect("SECRET must be set");
  env::var("MAILER_FROM_ADDRESS").expect("MAILER_FROM_ADDRESS must be set");
  env::var("MAILER_MAIL_SERVER").expect("MAILER_MAIL_SERVER must be set");
  env::var("MAILER_USERNAME").expect("MAILER_USERNAME must be set");
  env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD must be set");

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

  server::new(move || {
    App::with_state(State {
      db: addr.clone(),
      #[cfg(not(test))]
      #[cfg(feature = "mq")]
      jobs: JobQueue::clone(&producer),
    })
      .configure(init_middleware)
      .configure(handlers::handlers)
      .boxed()
  })
    .bind("127.0.0.1:3000")
    .expect("Unable to bind to port")
    .start();

  let _ = sys.run();
}

// TODO: Test
// - Routes
// - Template <a> tags
