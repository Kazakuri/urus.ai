// https://github.com/diesel-rs/diesel/pull/1787#issuecomment-445565553
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_enum;

#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod schema;
