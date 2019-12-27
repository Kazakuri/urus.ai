use actix_web::error::{JsonPayloadError, UrlencodedError};
use actix_web::{error, http, HttpResponse};
use std::error::Error;
use thiserror::Error;
use tokio_postgres::error::{DbError, SqlState};

#[derive(Error, Debug, PartialEq)]
pub enum UserError {
  // ==================================
  // Database Errors
  // ==================================
  #[error("{} is already in use!", field)]
  DuplicateValue { field: String },

  #[error("{} does not exist!", field)]
  UnknownValue { field: String },

  #[error("{} is not valid!", field)]
  InvalidValue { field: String },

  #[error("Not found.")]
  NotFound,

  // ==================================
  // Session Errors
  // ==================================
  #[error("Invalid login.")]
  LoginError,

  #[error("Email not verified")]
  EmailNotVerified,

  // ==================================
  // User Create Errors
  // ==================================
  #[error("Password too short. Passwords should contain at least 8 characters.")]
  PasswordTooShort,

  #[error("Password not complex enough. Passwords should contain at least one lowercase letter, one uppercase letter, one number, and one symbol."
  )]
  PasswordNotComplex,

  // ==================================
  #[error("Invalid characters in URL.")]
  InvalidCharactersInURL,

  #[error("The provided link looks like it's already shortened.")]
  LinkAlreadyShortened,

  #[error("That doesn't look like a URL, try again.")]
  InvalidLink,

  #[error("An internal error occurred. Please try again later.")]
  InternalError,

  #[error("Bad Request.")]
  BadRequest,
}

impl error::ResponseError for UserError {
  fn error_response(&self) -> HttpResponse {
    error!("{}", *self);
    match *self {
      UserError::InternalError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
      _ => HttpResponse::new(http::StatusCode::BAD_REQUEST),
    }
  }
}

impl From<JsonPayloadError> for UserError {
  fn from(_err: JsonPayloadError) -> UserError {
    UserError::InternalError
  }
}

impl From<UrlencodedError> for UserError {
  fn from(_err: UrlencodedError) -> UserError {
    UserError::BadRequest
  }
}

impl From<std::io::Error> for UserError {
  fn from(_err: std::io::Error) -> UserError {
    UserError::InternalError
  }
}

impl From<deadpool::PoolError<tokio_postgres::error::Error>> for UserError {
  fn from(err: deadpool::PoolError<tokio_postgres::error::Error>) -> UserError {
    match err {
      deadpool::PoolError::Timeout(_) => UserError::InternalError,
      deadpool::PoolError::Backend(e) => e.into(),
    }
  }
}

impl From<tokio_postgres::error::Error> for UserError {
  fn from(err: tokio_postgres::error::Error) -> UserError {
    let db_error = err.source().and_then(|e| e.downcast_ref::<DbError>());

    let code = db_error.map(DbError::code);

    let constraint = db_error.and_then(DbError::constraint).unwrap_or("Unknown").to_string();

    match code {
      Some(s) => {
        // TODO: Is there a better way to write this?
        // Matching against SqlState::UNIQUE_VIOLATION says:
        //   std::borrow::Cow must be annotated with #[derive(PartialEq, Eq)]`
        if s.code() == SqlState::UNIQUE_VIOLATION.code() {
          return UserError::DuplicateValue { field: constraint };
        }

        if s.code() == SqlState::FOREIGN_KEY_VIOLATION.code() {
          return UserError::UnknownValue { field: constraint };
        }

        if s.code() == SqlState::CHECK_VIOLATION.code() {
          return UserError::InvalidValue { field: constraint };
        }

        debug!("{:?}", err);
        UserError::InvalidValue { field: constraint }
      }
      None => UserError::InternalError,
    }
  }
}
