use askama::Template;

use urusai_lib::models::user::User;
use urusai_lib::models::user_token::UserToken;

#[derive(Template)]
#[template(path = "email/activation.html")]
pub struct Activation<'a> {
  pub user: &'a User,
  pub token: &'a UserToken,
}
