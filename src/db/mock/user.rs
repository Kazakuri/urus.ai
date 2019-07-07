/// Functions to create and persist a User in the database.
pub mod create;

/// Functions to read a User and his profile information from the database.
pub mod read_profile;

/// Functions to read a User from the database.
pub mod read;

/// Functions to trigger the validation of a User in the database.
pub mod verify;

pub use self::create::create;
pub use self::read::read;
pub use self::read_profile::read_profile;
pub use self::verify::verify;
