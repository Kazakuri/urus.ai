use urusai_lib::models::user::User;
use bcrypt;
use sodiumoxide::crypto::pwhash::argon2id13;
use crate::errors::UserError;
use lazy_static::lazy_static;
use regex::RegexSet;

lazy_static! {
    static ref REGEX_SET: RegexSet = RegexSet::new(&[r"[A-Z]", r"[a-z]", r"[\d]", r"\W",])
        .expect("Unable to create regex set for password complexity validation");
}

pub fn validate_and_hash_password(password: String) -> Result<String, UserError> {
    if password.len() < 8 {
        return Err(UserError::PasswordTooShort);
    }

    if REGEX_SET.matches(&password).len() != REGEX_SET.len() {
        return Err(UserError::PasswordNotComplex);
    }

    let pwh = argon2id13::pwhash(
        &password.into_bytes()[..],
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .expect("Unable to hash password");

    let hashed_password =
        String::from_utf8(pwh[..].to_vec()).expect("Unable to parse hashed password");

    // argon2 passwords are padded by null bytes and Postgres can't store null bytes, so we strip them
    Ok(hashed_password.trim_matches(char::from(0)).to_string())
}