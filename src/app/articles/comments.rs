use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments index")
}

pub async fn create() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments create")
}

pub async fn delete() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments delete")
}
