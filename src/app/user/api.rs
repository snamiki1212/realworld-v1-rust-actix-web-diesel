use super::model::{UpdatableUser, User};
use super::{request, response::UserResponse};
use crate::error::AppError;
use crate::middleware::auth;
use crate::middleware::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<request::Signin>,
) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let (user, token) = User::signin(&conn, &form.user.email, &form.user.password)?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::Signup>,
) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let (user, token) = User::signup(
        &conn,
        &form.user.email,
        &form.user.username,
        &form.user.password,
    )?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn me(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let user = auth::access_auth_user(&req)?;
    let token = user.generate_token()?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::Update>,
) -> Result<HttpResponse, AppError> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let user = User::update(
        &conn,
        auth_user.id,
        UpdatableUser {
            email: form.user.email.clone(),
            username: form.user.username.clone(),
            password: form.user.password.clone(),
            image: form.user.image.clone(),
            bio: form.user.bio.clone(),
        },
    )?;
    let token = &user.generate_token()?;
    let res = UserResponse::from((user, token.to_string()));
    Ok(HttpResponse::Ok().json(res))
}
