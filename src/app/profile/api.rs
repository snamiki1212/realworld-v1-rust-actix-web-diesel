use super::service;
use crate::app::profile;
use crate::middleware::auth::access_auth_user;
use crate::middleware::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let me = access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let _username = path.into_inner();

    let profile = service::fetch_by_name(
        &conn,
        &service::FetchProfileByName {
            me: me.to_owned(),
            username: _username,
        },
    )?;

    let res = profile::response::ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let user = access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let username = path.into_inner();
    let profile = user.follow(&conn, &username)?;
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let user = access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let username = path.into_inner();
    let profile = user.unfollow(&conn, &username)?;
    Ok(HttpResponse::Ok().json(profile))
}
