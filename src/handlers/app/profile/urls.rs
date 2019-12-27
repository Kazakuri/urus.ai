use actix_identity::Identity;
use actix_web::web::Query;
use actix_web::{web::Data, FromRequest, HttpRequest, HttpResponse};
use askama::Template;
use std::collections::HashMap;
use uuid::Uuid;

use crate::db::user::ReadUserProfile;
use crate::errors::UserError;
use crate::templates::ProfileURLs;
use crate::State;

pub async fn urls(id: Identity, state: Data<State>, req: HttpRequest) -> Result<HttpResponse, UserError> {
  if let Some(id) = id.identity() {
    let db = state.db.clone();
    let query = Query::<HashMap<String, String>>::extract(&req).await;

    let page = if let Ok(q) = query {
      match q.get("page").unwrap_or(&"1".to_string()).parse::<i64>() {
        Ok(p) => p,
        Err(_) => 1,
      }
    } else {
      1
    };

    let user_info = ReadUserProfile {
      id: Uuid::parse_str(&id).expect("Unable to parse UUID"),
      page,
    };

    let profile = crate::db::user::read_profile(&db, user_info).await;

    return match profile {
      Ok((user, links, pages)) => {
        let page_list = generate_page_list(page, pages);

        let previous_page = if page > 1 { Some(page - 1) } else { None };

        let next_page = if page < pages { Some(page + 1) } else { None };

        Ok(
          HttpResponse::Ok().content_type("text/html").body(
            ProfileURLs {
              user: &Some(user),
              links: &links,
              message: None,
              page: &page,
              next_page: &next_page,
              previous_page: &previous_page,
              pages: &page_list,
            }
            .render()
            .expect("Unable to render profile URL page"),
          ),
        )
      }
      Err(_e) => Ok(HttpResponse::SeeOther().header("Location", "/").finish()),
    };
  }

  Ok(HttpResponse::SeeOther().header("Location", "/").finish())
}

// When iterating the range, we only care about these two cases:
//  Current value is 2 ahead of the last value, add the inbetween value to the list
//  Current value has skipped more than that, add a "..."
#[allow(clippy::else_if_without_else)]

fn generate_page_list(current_page: i64, total_pages: i64) -> Vec<Option<i64>> {
  let mut page_list: Vec<Option<i64>> = vec![];

  let delta = 4;
  let left = current_page - delta;
  let right = current_page + delta;

  let mut range: Vec<i64> = vec![];

  for i in 1..=total_pages {
    if i == 1 || i == total_pages || (i >= left && i <= right) {
      range.push(i);
    }
  }

  let mut l: Option<i64> = None;

  for i in range {
    if let Some(v) = l {
      if i - v == 2 {
        page_list.push(Some(v + 1));
      } else if i - v != 1 {
        page_list.push(None);
      }
    }

    page_list.push(Some(i));
    l = Some(i);
  }

  page_list
}
