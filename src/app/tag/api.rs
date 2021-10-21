// #[macro_use]
extern crate serde_json;

use super::service;
use crate::utils::db::DbPool;
use actix_web::{get, web, HttpResponse, Responder};

#[get("")]
pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let list = web::block(move || service::list(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().json(e.to_string())
            // HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(list))
}
