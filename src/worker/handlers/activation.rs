use faktory::Job;
use std::io::{ Result, Error, ErrorKind };
use lettre_email::EmailBuilder;
use askama::Template;
use native_tls::{ TlsConnector, Protocol };
use lettre::smtp::authentication::{ Credentials, Mechanism };
use lettre::{ EmailTransport, ClientTlsParameters, ClientSecurity };
use lettre::smtp::{ SmtpTransportBuilder, ConnectionReuseParameters };
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

      let email = EmailBuilder::new()
        .to(user.email)
        .from(from_address)
        .subject("Welcome to urus.ai!")
        .alternative(html, text)
        .build()
        .unwrap();

      let mut tls_builder = TlsConnector::builder().unwrap();
      tls_builder.supported_protocols(&[Protocol::Tlsv12]).unwrap();

      let tls_parameters = ClientTlsParameters::new(
        mail_server.to_string(),
        tls_builder.build().unwrap()
      );

      let mut mailer = SmtpTransportBuilder::new(
        (&mail_server[..], 465),
        ClientSecurity::Wrapper(tls_parameters)
      )
        .expect("Failed to create transport")
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(username, password))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();

      let result = mailer.send(&email);

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
