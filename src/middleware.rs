use actix_web::middleware::identity::{ CookieIdentityPolicy, IdentityService };
use actix_web::App;
use actix_web::middleware::Logger;
use chrono::Duration;
use std::env;

use crate::State;

/// Takes in an Actix App and returns it with our middleware appended.
pub fn init_middleware(app: App<State>) -> App<State> {
  let secret = env::var("SECRET").expect("SECRET must be set");
  let _domain = env::var("DOMAIN").expect("DOMAIN must be set");

  app
    .middleware(Logger::new("\"%r\" %s %b %Dms"))
    .middleware(IdentityService::new(
      CookieIdentityPolicy::new(secret.as_bytes())
        .name("auth")
        .path("/")
        //.domain(domain)
        .max_age(Duration::days(1))
        .secure(false)
    ))
}
