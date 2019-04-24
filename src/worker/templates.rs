use askama::Template;

use urusai_lib::models::user::User;

#[derive(Template)]
#[template(path = "email/activation.html")]
pub struct Activation<'a> {
  pub user: &'a User,
}
