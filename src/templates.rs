use askama::Template;

use urusai_lib::models::message::{ Message, MessageType };
use urusai_lib::models::short_url::ShortURL;
use urusai_lib::models::user::User;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct Index<'a> {
  pub user: &'a Option<User>,

  pub message: Option<&'a Message<'a>>,

  pub url: Option<&'a str>,
}

#[derive(Template, Debug)]
#[template(path = "login.html")]
pub struct Login<'a> {
  pub user: &'a Option<User>,

  pub message: Option<&'a Message<'a>>,
}

#[derive(Template, Debug)]
#[template(path = "register.html")]
pub struct Register<'a> {
  pub user: &'a Option<User>,

  pub message: Option<&'a Message<'a>>,
}

#[derive(Template, Debug)]
#[template(path = "profile/urls.html")]
pub struct ProfileURLs<'a> {
  pub user: &'a Option<User>,

  pub message: Option<&'a Message<'a>>,

  pub links: &'a Vec<ShortURL>,

  pub page: &'a i64,

  pub previous_page: &'a Option<i64>,

  pub next_page: &'a Option<i64>,

  pub pages: &'a Vec<Option<i64>>,
}

#[derive(Template, Debug)]
#[template(path = "profile/account.html")]
pub struct ProfileAccount<'a> {
  pub user: &'a Option<User>,

  pub message: Option<&'a Message<'a>>,
}
