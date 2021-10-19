use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("")]
pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("show_articles")
}

#[get("/feed")]
pub async fn feed() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("feed of articles")
}

#[get("/{id}")]
pub async fn show() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("detail_article")
}

#[post("")]
pub async fn create() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("create_article")
}

#[put("/{id}")]
pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("update_article")
}

#[delete("")]
pub async fn delete() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("delete_article")
}
