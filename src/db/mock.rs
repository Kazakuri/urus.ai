/// A production-ready backend implementation that processes messages into a Postgres database
use super::Repository;

use actix::prelude::*;

/// Functions to handle interacting with mock User sessions.
mod session;

/// Functions to handle interacting with mock ShortURL objects.
mod url;

/// Functions to handle interacting with mock User objects.
mod user;

/// Dummy database object
pub struct MockDatabase {}

impl MockDatabase {
    /// Creates a new Database object
    pub fn new() -> Self {
        MockDatabase {}
    }
}

use super::messages::session::CreateSession;
use super::messages::url::{CreateURL, ReadURL};
use super::messages::user::{CreateUser, ReadUser, ReadUserProfile, VerifyUser};

impl Repository for MockDatabase {
    fn create_session(&self, msg: CreateSession) -> <CreateSession as Message>::Result {
        self::session::create(&msg)
    }

    fn create_url(&self, msg: CreateURL) -> <CreateURL as Message>::Result {
        self::url::create(msg)
    }

    fn read_url(&self, msg: ReadURL) -> <ReadURL as Message>::Result {
        self::url::read(&msg)
    }

    fn create_user(&self, msg: CreateUser) -> <CreateUser as Message>::Result {
        self::user::create(msg)
    }

    fn read_user(&self, msg: ReadUser) -> <ReadUser as Message>::Result {
        self::user::read(&msg)
    }

    fn read_user_profile(&self, msg: ReadUserProfile) -> <ReadUserProfile as Message>::Result {
        self::user::read_profile(&msg)
    }

    fn verify_user(&self, msg: VerifyUser) -> <VerifyUser as Message>::Result {
        self::user::verify(&msg)
    }
}
