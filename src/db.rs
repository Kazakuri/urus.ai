use actix::prelude::*;
use failure::Error;
use std::env;

/// Actor messages to send to the Repository
pub mod messages;

use self::messages::session::{ CreateSession };
use self::messages::url::{ CreateURL, ReadURL };
use self::messages::user::{ CreateUser, ReadUserProfile, ReadUser, VerifyUser };

/// Production ready implementation of a Postgres database connection.
mod implementation;

#[cfg(test)]
/// Mock database to be used for testing.
mod mock;

/// Repository for querying information about the application through Actix worker messages.
pub trait Repository: Sync + Send {
  /// Logs a user in from a CreateSession message
  fn create_session(&self, msg: CreateSession) -> <CreateSession as Message>::Result;

  /// Creates a ShortURL from a CreateURL message
  fn create_url(&self, msg: CreateURL) -> <CreateURL as Message>::Result;
  /// Reads a ShortURL from a ReadURL message
  fn read_url(&self, msg: ReadURL) -> <ReadURL as Message>::Result;

  /// Creates a User from a CreateUser message
  fn create_user(&self, msg: CreateUser) -> <CreateUser as Message>::Result;
  /// Reads a User from a ReadUser message
  fn read_user(&self, msg: ReadUser) -> <ReadUser as Message>::Result;
  /// Reads a User 's profile from a ReadUserProfile message
  fn read_user_profile(&self, msg: ReadUserProfile) -> <ReadUserProfile as Message>::Result;
  /// Verifies a User's email from a VerifyUser message
  fn verify_user(&self, msg: VerifyUser) -> <VerifyUser as Message>::Result;
}

/// A boxed type for any struct that implements the Repository trait.
pub type DataRepository = Box<dyn Repository>;

/// Message executor context for a struct that implements the Repository trait.
pub struct DbExecutor(pub DataRepository);

impl Actor for DbExecutor {
  type Context = SyncContext<Self>;
}

/// Returns a repository to a production-ready database.
///
/// Will panic if the environment variable `DATABASE_URL` is not set to a valid Postgres URI.
pub fn database() -> Result<DataRepository, Error> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  Ok(
    Box::new(
      implementation::Database::new(database_url).expect("Could not connect to the database")
    )
  )
}

#[cfg(test)]
pub fn mock() -> Result<DataRepository, !> {
  Ok(Box::new(mock::MockDatabase::new()))
}
