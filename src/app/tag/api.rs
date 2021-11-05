extern crate serde_json;
use super::model::Tag;
use super::response;
use crate::error::AppError;
use crate::AppState;
use actix_web::{error::Error as ActixWebErr, web, HttpResponse};
use anyhow::{Context, Result};

pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let list = Tag::list(&conn)?;

    let res = response::TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
