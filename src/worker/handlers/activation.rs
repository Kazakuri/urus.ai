use faktory::Job;
use std::io::{ Result, Error, ErrorKind };
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use askama::Template;
use std::env;

use crate::templates::Activation;

pub fn activation(job: &Job) -> Result<()> {
  println!("{:?}", job);

  if let Some(body) = job.args().first() {
    if let Ok(user) = serde_json::from_value(body.to_owned()) {
      let html = Activation {
        user: &user,
      }.render().unwrap();

      let from_address = env::var("MAILER_FROM_ADDRESS").unwrap();
      let mail_server = env::var("MAILER_MAIL_SERVER").unwrap();
      let username = env::var("MAILER_USERNAME").unwrap();
      let password = env::var("MAILER_PASSWORD").unwrap();

      let text = format!("Welcome to urus.ai!\n\n \
                  Please visit the link below to verify your account and start using urus.ai immediately.\n \
                  https://urus.ai/verify/{}", &user.id);

      let email = Email::builder()
        .to(user.email)
        .from(from_address)
        .subject("Welcome to urus.ai!")
        .alternative(html, text)
        .build()
        .unwrap();

      let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();

      let result = mailer.send(email.into());

      if result.is_err() {
        error!("{:?}", result);
      }

      mailer.close();

      Ok(())
    } else {
      Err(Error::new(ErrorKind::InvalidInput, "Invalid user object"))
    }
  } else {
    Err(Error::new(ErrorKind::InvalidInput, "Missing user"))
  }
}
