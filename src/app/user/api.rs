use super::model::{UpdateUser, User};
use super::{request, response::UserResponse};
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn signin(state: web::Data<AppState>, form: web::Json<request::Signin>) -> ApiResponse {
    let conn = state.get_conn()?;
    let (user, token) = User::signin(&conn, &form.user.email, &form.user.password)?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(state: web::Data<AppState>, form: web::Json<request::Signup>) -> ApiResponse {
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

pub async fn me(req: HttpRequest) -> ApiResponse {
    let user = auth::get_current_user(&req)?;
    let token = user.generate_token()?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::Update>,
) -> ApiResponse {
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let user = User::update(
        &conn,
        current_user.id,
        UpdateUser {
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
