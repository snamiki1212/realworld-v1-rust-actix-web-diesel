extern crate serde_json;
use super::entities::Tag;
use super::presenters::TagsResponse;
use crate::appv2::drivers::middlewares::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpResponse};

pub async fn index(state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let list = Tag::fetch(conn)?;
    let res = TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
