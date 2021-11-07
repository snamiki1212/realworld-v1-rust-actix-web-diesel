extern crate serde_json;
use super::model::Tag;
use super::response;
use crate::middleware::state::AppState;
use actix_web::{web, HttpResponse};
use anyhow::Result;

pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let conn = state.get_conn()?;
    let list = Tag::list(&conn)?;
    let res = response::TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
