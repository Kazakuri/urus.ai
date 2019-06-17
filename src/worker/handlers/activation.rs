use faktory::Job;
use std::io::{ Result, Error, ErrorKind };
use lettre::{ SmtpClient, Transport, smtp::ClientSecurity };
use lettre_email::Email;
use askama::Template;
use std::env;

use urusai_lib::models::user::User;
use urusai_lib::models::user_token::UserToken;

use crate::templates::Activation;

pub fn activation(job: &Job) -> Result<()> {
  match job.args() {
    [] => Err(Error::new(ErrorKind::InvalidInput, "Missing user")),
    [_] => Err(Error::new(ErrorKind::InvalidInput, "Missing user token")),
    [user_body, token_body] => {
      let user_args: String = serde_json::from_value(user_body.to_owned()).expect("Unable to parse job");
      let token_args: String = serde_json::from_value(token_body.to_owned()).expect("Unable to parse job");

      match (serde_json::from_str::<User>(&user_args), serde_json::from_str::<UserToken>(&token_args)) {
        (Err(_), _) => Err(Error::new(ErrorKind::InvalidInput, "Invalid user object")),
        (_, Err(_)) => Err(Error::new(ErrorKind::InvalidInput, "Invalid user token object")),
        (Ok(user), Ok(token)) => {
          let html = Activation {
            token: &token,
            user: &user,
          }.render().unwrap();

          let from_address = env::var("MAILER_FROM_ADDRESS").unwrap();
          let mail_server = env::var("MAILER_MAIL_SERVER").unwrap();
          let username = env::var("MAILER_USERNAME").unwrap();
          let password = env::var("MAILER_PASSWORD").unwrap();

          let text = format!("Welcome to urus.ai!\n\n \
                      Please visit the link below to verify your account and start using urus.ai immediately.\n \
                      https://urus.ai/verify/{}/{}", &token.user_id, &token.id);

          let email = Email::builder()
            .to(user.email)
            .from(from_address)
            .subject("Welcome to urus.ai!")
            .alternative(html, text)
            .build()
            .unwrap();

          // TODO: Use our environment variables instead of the hard-coded test server.
          let mut mailer = SmtpClient::new("127.0.0.1:1025", ClientSecurity::None).unwrap().transport();

          let result = mailer.send(email.into());

          if result.is_err() {
            error!("{:?}", result);
          }

          mailer.close();
          Ok(())
        }
      }
    },
    _ => Err(Error::new(ErrorKind::InvalidInput, "Too many arguments")),
  }
}
