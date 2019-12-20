use chrono::NaiveDateTime;
use uuid::Uuid;

use urusai_lib::models::short_url::ShortURL;

use crate::db::Pool;
use crate::errors::UserError;

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadURL {
  pub slug: String,
}

pub async fn read(pool: &Pool, msg: ReadURL) -> Result<ShortURL, UserError> {
  let statement = "
    SELECT
      id,
      user_id,
      slug,
      url,
      visits,
      created_at,
      updated_at
    FROM urls
    WHERE slug = $1
  ";

  let mut client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;

  debug!("Looking for {}", msg.slug);

  let row = client.query_one(&prepared_statement, &[&msg.slug]).await?;

  let url = ShortURL {
    id: row.try_get::<_, Uuid>(0)?,
    user_id: row.try_get::<_, Option<Uuid>>(1)?,
    slug: row.try_get::<_, String>(2)?,
    url: row.try_get::<_, String>(3)?,
    visits: row.try_get::<_, i64>(4)?,
    created_at: row.try_get::<_, NaiveDateTime>(5)?,
    updated_at: row.try_get::<_, NaiveDateTime>(6)?,
  };

  Ok(url)
}
