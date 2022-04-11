extern crate serde_json;
use super::model::Tag;
use super::response::TagsResponse;
use crate::{error::AppError, middleware::state::AppState};
use actix_web::{web, HttpResponse};

pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let list = Tag::fetch_list(&conn)?;
    let res = TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
