use deadpool_postgres::Manager;
use std::env;
use tokio_postgres::{Config, NoTls};

pub use deadpool_postgres::Pool;

pub mod session;
pub mod url;
pub mod user;

pub fn create_pool() -> Pool {
  let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let cfg = url.parse::<Config>().expect("Unable to parse DATABASE_URL");

  let mgr = Manager::new(cfg, NoTls);

  Pool::new(mgr, 16)
}
