use askama::Template;

use urusai_lib::models::message::{Message, MessageType};

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct Index<'a> {
  pub message: Option<&'a Message<'a>>,

  pub url: Option<&'a str>,
}
