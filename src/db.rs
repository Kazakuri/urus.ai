use deadpool_postgres::Manager;
use std::env;
use tokio_postgres::{Config, NoTls};

pub type Pool = deadpool_postgres::Pool<NoTls>;

pub mod url;

pub fn create_pool() -> Pool {
  let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let cfg = url.parse::<Config>().expect("Unable to parse DATABASE_URL");

  let mgr = Manager::new(cfg, NoTls);

  Pool::new(mgr, 16)
}
