use actix_web::{HttpResponse, Responder};

pub async fn favorite() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("favorite")
}

pub async fn unfavorite() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("unfavorite")
}
