/// Functions to create and persist a ShortURL in the database.
pub mod create;

/// Functions to read a ShortURL from the database.
pub mod read;

pub use self::create::create;
pub use self::read::read;
