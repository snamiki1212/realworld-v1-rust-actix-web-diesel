use crate::app::profile;
use crate::app::user::model::User;
use crate::AppState;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

type UsernameSlug = String;

#[get("/{username}")]
pub async fn show(state: web::Data<AppState>, path: web::Path<UsernameSlug>) -> impl Responder {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let username = path.into_inner();
    let profile = profile::model::Profile::find_by_name(&conn, &username)
        .expect("couldn't find profile by name");
    let res = profile::response::ProfileResponse::from(profile);
    HttpResponse::Ok().json(res)
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
