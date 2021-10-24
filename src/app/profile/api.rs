use actix_web::{delete, get, post, web, HttpResponse, Responder};

type UsernameSlug = String;

#[get("/{username}")]
pub async fn show(path: web::Path<UsernameSlug>) -> impl Responder {
    // TODO:
    println!("___");
    println!("---show / {:?}", path);
    let path = path.into_inner();
    let msg = format!("path is {}", path);
    HttpResponse::Ok().body(msg)
}

#[post("/{username}/follow")]
pub async fn follow() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("profile follow")
}

#[delete("/{username}/unfollow")]
pub async fn unfollow() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("profile unfollow")
}
