#[macro_use]
extern crate log;

use faktory::ConsumerBuilder;
use dotenv::dotenv;

pub mod handlers;
pub mod templates;

fn main() {
  dotenv().ok();

  let mut c = ConsumerBuilder::default();

  c.register("send_activation_email", |job| handlers::activation(&job));

  let mut c = c.connect(None).unwrap();

  println!("Connected to faktory!");

  if let Err(e) = c.run(&["default"]) {
    println!("Worker failed: {}", e);
  }
}
