extern crate serde_json;
use super::model::Tag;
use super::response::TagsResponse;
use crate::middleware::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpResponse};

pub async fn index(state: web::Data<AppState>) -> ApiResponse {
    let conn = state.get_conn()?;
    let list = Tag::fetch(&conn)?;
    let res = TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
