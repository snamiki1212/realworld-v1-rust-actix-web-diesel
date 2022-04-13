use super::response::ProfileResponse;
use super::service;
use crate::error::AppError;
use crate::middleware::auth::access_auth_user;
use crate::middleware::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, AppError> {
    let auth_user = access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let _username = path.into_inner();
    let profile = service::fetch_by_name(
        &conn,
        &service::FetchProfileByName {
            me: auth_user,
            username: _username,
        },
    )?;
    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, AppError> {
    let auth_user = access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let username = path.into_inner();
    let profile = auth_user.follow(&conn, &username)?;
    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, AppError> {
    let auth_user = access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let username = path.into_inner();
    let profile = auth_user.unfollow(&conn, &username)?;
    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}
