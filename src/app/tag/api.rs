extern crate serde_json;
use super::model::Tag;
use super::response;
use crate::error::AppError;
use crate::AppState;
use actix_web::{error::Error as ActixWebErr, web, HttpResponse};
use anyhow::{Context, Result};

pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, ActixWebErr> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let list = Tag::list(&conn)
        .map_err::<AppError, _>(|_| AppError::HogeError("test".to_string()).into())?;
    // let list = web::block(move || Tag::list(&conn)).await.map_err(|e| {
    //     eprintln!("{}", e);
    //     HttpResponse::InternalServerError().json(e.to_string())
    // })?;

    let res = response::TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
