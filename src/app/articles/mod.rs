use actix_web::{HttpResponse, Responder};
pub mod comments;
pub mod favorites;

pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("show_articles")
}

pub async fn feed() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("feed of articles")
}

pub async fn show() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("detail_article")
}

pub async fn create() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("create_article")
}

pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("update_article")
}

pub async fn delete() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("delete_article")
}
