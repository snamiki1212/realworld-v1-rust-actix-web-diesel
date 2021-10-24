use crate::app::profile::response;
use crate::app::user::model::User;
use crate::AppState;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

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
pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> impl Responder {
    let head = req.head();
    let extensions = head.extensions();
    let user = extensions
        .get::<User>()
        .expect("couldn't get user on req extension.");

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let username = path.into_inner();

    let profile = user.follow(&conn, &username).expect("couldn't follow user");
    HttpResponse::Ok().json(profile)
}

#[delete("/{username}/follow")]
pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> impl Responder {
    let head = req.head();
    let extensions = head.extensions();
    let user = extensions
        .get::<User>()
        .expect("couldn't get user on req extension.");

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let username = path.into_inner();

    let profile = user
        .unfollow(&conn, &username)
        .expect("couldn't unfollow user");
    HttpResponse::Ok().json(profile)
}
