use super::response::ProfileResponse;
use super::service;
use crate::error::AppError;
use crate::middleware::{auth, state::AppState};
use actix_web::{web, HttpRequest, HttpResponse};

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let _username = path.into_inner();
    let profile = service::fetch_by_name(
        &conn,
        &service::FetchProfileByName {
            current_user,
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
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let username = path.into_inner();
    let profile = current_user.follow(&conn, &username)?;
    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let username = path.into_inner();
    let profile = current_user.unfollow(&conn, &username)?;
    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}
