use super::service;
use crate::app::profile;
use crate::app::user::model::User;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> impl Responder {
    let head = req.head();
    let extensions = head.extensions();
    let me = extensions
        .get::<User>()
        .expect("couldn't get user on req extension.");

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let _username = path.into_inner();

    let profile = service::fetch(
        &conn,
        &service::FetchProfile {
            me: me.to_owned(),
            username: _username,
        },
    );

    let res = profile::response::ProfileResponse::from(profile);
    HttpResponse::Ok().json(res)
}

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
