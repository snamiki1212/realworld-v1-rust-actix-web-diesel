use actix_web::{delete, get, post, HttpResponse, Responder};

#[get("")]
pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments index")
}

#[post("")]
pub async fn create() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments create")
}

#[delete("")]
pub async fn delete() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments delete")
}
