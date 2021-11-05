use crate::app::user::model::{UpdatableUser, User};
use crate::app::user::{request, response};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<request::Signin>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (user, token) =
        web::block(move || User::signin(&conn, &form.user.email, &form.user.password))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().json(e.to_string())
            })?;
    let res = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::Signup>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (user, token) = web::block(move || {
        User::signup(
            &conn,
            &form.user.email,
            &form.user.username,
            &form.user.password,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;

    let res = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn me(req: HttpRequest) -> HttpResponse {
    let user = auth::access_auth_user(&req).expect("could not fetch auth.");
    let user = response::UserResponse::from((user.to_owned(), user.generate_token()));
    HttpResponse::Ok().json(user)
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::Update>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = form.user.clone();

    let user = UpdatableUser {
        email: user.email,
        username: user.username,
        password: user.password,
        image: user.image,
        bio: user.bio,
    };
    let user = web::block(move || User::update(&conn, auth_user.id, user))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        })?;

    let token = &user.generate_token();
    let res = response::UserResponse::from((user, token.to_string()));

    Ok(HttpResponse::Ok().json(res))
}
