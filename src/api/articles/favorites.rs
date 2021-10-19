use actix_web::{delete, post, HttpResponse, Responder};

#[post("")]
pub async fn favorite() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("favorite")
}

#[delete("")]
pub async fn unfavorite() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("unfavorite")
}
