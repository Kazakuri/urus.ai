/// A production-ready backend implementation that processes messages into a Postgres database

use super::Repository;

use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ ConnectionManager, Pool, PooledConnection };

/// Functions to handle interacting with User sessions.
mod session;

/// Functions to handle interacting with ShortURL objects in the database.
mod url;

/// Functions to handle interacting with User objects in the database.
mod user;

use crate::errors::UserError;

/// A pool of Postgres connections
type PostgresPool = Pool<ConnectionManager<PgConnection>>;

/// A pooled connection to a Postgres database
type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// Repository wrapper for a pool of Postgres connections
pub struct Database {
  /// An r2d2 Pool for Postgres connections
  pool: PostgresPool,
}

impl Database {
  /// Creates a new Database object with a pool of Postgres connections to the provided uri
  pub fn new(database_uri: String) -> Result<Self, r2d2::Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    let pool = Pool::new(manager)?;

    Ok(Database { pool })
  }

  /// Fetches a single connection from the pool
  pub fn connection(&self) -> Connection {
    self.pool.get().map_err(|_| UserError::InternalError)
      .expect("Unable to fetch a connection from the pool")
  }
}

use super::messages::session::{ CreateSession };
use super::messages::url::{ CreateURL, ReadURL };
use super::messages::user::{ CreateUser, ReadUserProfile, ReadUser, VerifyUser };

impl Repository for Database {
  fn create_session(&self, msg: CreateSession) -> <CreateSession as Message>::Result {
    self::session::create(&self.connection(), &msg)
  }

  fn create_url(&self, msg: CreateURL) -> <CreateURL as Message>::Result {
    self::url::create(&self.connection(), msg)
  }

  fn read_url(&self, msg: ReadURL) -> <ReadURL as Message>::Result {
    self::url::read(&self.connection(), &msg)
  }

  fn create_user(&self, msg: CreateUser) -> <CreateUser as Message>::Result {
    self::user::create(&self.connection(), msg)
  }

  fn read_user(&self, msg: ReadUser) -> <ReadUser as Message>::Result {
    self::user::read(&self.connection(), &msg)
  }

  fn read_user_profile(&self, msg: ReadUserProfile) -> <ReadUserProfile as Message>::Result {
    self::user::read_profile(&self.connection(), &msg)
  }

  fn verify_user(&self, msg: VerifyUser) -> <VerifyUser as Message>::Result {
    self::user::verify(&self.connection(), &msg)
  }
}
