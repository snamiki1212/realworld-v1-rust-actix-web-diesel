use actix_web::{get, post, delete,HttpResponse, Responder};

#[get("/{user_name}")]
pub async fn show() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("profile show")
}

#[post("/{user_name}/follow")]
pub async fn follow() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("profile follow")
}


#[delete("/{user_name}/unfollow")]
pub async fn unfollow() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("profile unfollow")
}

