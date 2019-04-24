use actix_web::{ HttpResponse, http, error };
use actix_web::error::{ JsonPayloadError, UrlencodedError };
use actix::MailboxError;

#[derive(Fail, Debug, PartialEq)]
/// User-facing error messages
pub enum UserError {
  // ==================================
  // Database Errors
  // ==================================

  #[fail(display="{} is already in use!", field)]
  /// The value being inserted violates a unique constraint in the database.
  DuplicateValue {
    /// The field that violated the constraint
    field: String
  },

  #[fail(display="{} does not exist!", field)]
  /// The value being inserted violates a foreign key constraint in the database.
  UnknownValue {
    /// The field that violated the foreign key constraint
    field: String
  },

  #[fail(display="{} is not valid!", field)]
  /// The value being inserted violates a check constraint in the database.
  InvalidValue {
    /// The field that violated the check constraint
    field: String
  },

  #[fail(display="Not found.")]
  /// The requested record doesn't exist in the database.
  NotFound,

  // ==================================
  // Session Errors
  // ==================================

  #[fail(display="Invalid login.")]
  /// The login information provided from the user was not valid.
  LoginError,

  #[fail(display="Email not verified")]
  /// The user tried to login without first verifying their account.
  EmailNotVerified,

  // ==================================
  // User Create Errors
  // ==================================

  #[fail(display="Password too short. Passwords should contain at least 8 characters.")]
  /// User supplied password failed our length test.
  PasswordTooShort,

  #[fail(display="Password not complex enough. Passwords should contain at least one lowercase letter, one uppercase letter, one number, and one symbol.")]
  /// User supplied password failed our complexity test.
  PasswordNotComplex,

  // ==================================

  #[fail(display="Invalid characters in URL.")]
  /// When creating a ShortURL, the user supplied non-URL-friendly characters.
  InvalidCharactersInURL,

  #[fail(display="The provided link looks like it's already shortened.")]
  /// When creating a ShortURL, the user supplied a URL that was already shortened.
  LinkAlreadyShortened,

  #[fail(display="That doesn't look like a URL, try again.")]
  /// When creating a ShortURL, the user supplied something that doesn't look like a URL.
  InvalidLink,

  #[fail(display="An internal error occurred. Please try again later.")]
  /// Something failed, but it wasn't the user's fault.
  InternalError,

  #[fail(display="Bad Request.")]
  /// The user provided a malformed request.
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

impl From<MailboxError> for UserError {
  fn from(_err: MailboxError) -> UserError {
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
