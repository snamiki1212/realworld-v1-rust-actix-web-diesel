use crate::app::user::model::{UpdatableUser, User};
use crate::app::user::{request, response};
use crate::middleware::auth;
use crate::middleware::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<request::Signin>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state.get_conn()?;
    let (user, token) = User::signin(&conn, &form.user.email, &form.user.password)?;
    let res = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::Signup>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state.get_conn()?;
    let (user, token) = User::signup(
        &conn,
        &form.user.email,
        &form.user.username,
        &form.user.password,
    )?;
    let res = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn me(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let user = auth::access_auth_user(&req)?;
    let token = user.generate_token()?;
    let user = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::Update>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let user = form.user.clone();
    let user = UpdatableUser {
        email: user.email,
        username: user.username,
        password: user.password,
        image: user.image,
        bio: user.bio,
    };
    let user = User::update(&conn, auth_user.id, user)?;
    let token = &user.generate_token()?;
    let res = response::UserResponse::from((user, token.to_string()));
    Ok(HttpResponse::Ok().json(res))
}
