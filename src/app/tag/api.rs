extern crate serde_json;

use super::model::Tag;
use crate::utils::db::DbPool;
use actix_web::{get, web, HttpResponse};

#[get("")]
pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let list = web::block(move || Tag::list(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;

    Ok(HttpResponse::Ok().json(list))
}
