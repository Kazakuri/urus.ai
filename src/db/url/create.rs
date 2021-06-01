use lazy_static::lazy_static;
use regex::Regex;

use crate::db::Pool;
use crate::errors::UserError;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateURL {
  pub url: String,
  pub slug: Option<String>,
}

lazy_static! {
  static ref URL_BLACKLIST: Vec<&'static str> = vec![
    "://urus.ai",
    "://polr.me",
    "://bit.ly",
    "://is.gd",
    "://tiny.cc",
    "://adf.ly",
    "://ur1.ca",
    "://goo.gl",
    "://ow.ly",
    "://j.mp",
    "://t.co",
  ];
}

lazy_static! {
  static ref URL_REGEX: Regex = Regex::new(
    r"^(http://www\.|https://www\.|http://|https://)?[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,5}(:[0-9]{1,5})?(/.*)?$"
  )
  .expect("Unable to compile regex");
}

pub async fn create(pool: &Pool, msg: CreateURL) -> Result<String, UserError> {
  let statement = "
    INSERT INTO urls (
      slug,
      url
    ) VALUES ($1, $2)
  ";

  let client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;

  let url = &msg.url;

  if URL_BLACKLIST.iter().any(|u| url.contains(u)) {
    debug!("Found an already shortened URL: {}", url);
    return Err(UserError::LinkAlreadyShortened);
  }

  if !URL_REGEX.is_match(url) {
    debug!("Found an invalid URL: {}", url);
    return Err(UserError::InvalidLink);
  }

  // Providing a URL slug is optional, so generate our own URL-friendly ID if they didn't provide one
  // At 10 IDs per hour, it'll take ~27 years to have a 1% probability of a collision for an 8 character nanoid
  let url_slug = match msg.slug {
    Some(url_slug) => {
      if url_slug.is_empty() {
        nanoid::nanoid!(8).to_string()
      } else {
        url_slug
      }
    }
    None => nanoid::nanoid!(8).to_string(),
  };

  debug!("Found URL slug: {}", url_slug);

  // Validate that our slug is URL-friendly
  if !url_slug.chars().all(|c| nanoid::alphabet::SAFE.iter().any(|x| *x == c)) {
    debug!("Found an invalid URL slug");
    return Err(UserError::InvalidCharactersInURL);
  }

  client.execute(&prepared_statement, &[&url_slug, &url]).await?;

  Ok(url_slug)
}
