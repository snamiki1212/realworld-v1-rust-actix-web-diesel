use actix_web::{get , HttpResponse, Responder};

#[get("")]
pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("tags")
}
