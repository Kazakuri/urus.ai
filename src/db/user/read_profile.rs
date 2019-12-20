use chrono::NaiveDateTime;
use futures::join;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

use urusai_lib::models::short_url::ShortURL;
use urusai_lib::models::user::User;

use crate::db::user::read::ReadUser;
use crate::db::Pool;
use crate::errors::UserError;

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct ReadUserProfile {
  pub id: Uuid,
  pub page: i64,
}

const COUNT_PER_PAGE: i64 = 20;

pub async fn read_profile(pool: &Pool, msg: ReadUserProfile) -> Result<(User, Vec<ShortURL>, i64), UserError> {
  let user_pool = pool.clone();

  let user = crate::db::user::read(&user_pool, ReadUser { id: msg.id });

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
    WHERE user_id = $1
    ORDER BY updated_at DESC
    LIMIT $2
    OFFSET $3
  ";

  let count_statement = "SELECT count(*) FROM urls WHERE user_id = $1";

  let mut client = pool.get().await?;
  let prepared_statement = client.prepare(statement).await?;
  let prepared_count_statement = client.prepare(count_statement).await?;

  debug!("Loading profile!");

  let page = msg.page - 1;
  let args: &[&(dyn ToSql + Sync)] = &[&msg.id, &COUNT_PER_PAGE, &page];
  let count_args: &[&(dyn ToSql + Sync)] = &[&msg.id];

  let (user, rows, count_row) = join!(
    user,
    client.query(&prepared_statement, args),
    client.query_one(&prepared_count_statement, count_args),
  );

  let count = count_row?.try_get::<_, i64>(0)?;
  let pages = count / COUNT_PER_PAGE + 1;

  let rows = rows?;

  let urls = rows
    .iter()
    .map(|row| ShortURL {
      id: row.get::<_, Uuid>(0),
      user_id: row.get::<_, Option<Uuid>>(1),
      slug: row.get::<_, String>(2),
      url: row.get::<_, String>(3),
      visits: row.get::<_, i64>(4),
      created_at: row.get::<_, NaiveDateTime>(5),
      updated_at: row.get::<_, NaiveDateTime>(6),
    })
    .collect::<Vec<ShortURL>>();

  Ok((user?, urls, pages))
}
