use bcrypt;
use sodiumoxide::crypto::pwhash::argon2id13;

pub fn verify_password(hash: &str, password: &str) -> bool {
  let mut bytes = hash.as_bytes().to_vec();

  if !bytes.starts_with(&b"$argon2id$"[..]) {
    return match bcrypt::verify(password, hash) {
      Ok(ok) => ok,
      Err(_) => false,
    };
  }

  // argon2 passwords are padded by null bytes and Postgres can't store null bytes, so we strip them
  // Here we re-pad the loaded password with null bytes to reverse the stripping we did when we generated the hash.
  bytes.resize(128, 0x00);

  let hash = argon2id13::HashedPassword::from_slice(&bytes[..])
    .expect("Could not resolve password_hash as a valid argon2id hash");

  argon2id13::pwhash_verify(&hash, &password.as_bytes()[..])
}
