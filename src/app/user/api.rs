use super::model::User;
use crate::utils::db::DbPool;
use actix_web::{get, post, put, web, HttpResponse, Responder};

#[post("/login")]
pub async fn signin() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users signin")
}

#[post("")]
pub async fn signup(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || User::signup(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("")]
pub async fn me() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users me")
}

#[put("")]
pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users update")
}
