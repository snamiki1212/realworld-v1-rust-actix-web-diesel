use crate::app::user::model::{UpdatableUser, User};
use crate::app::user::{request, response};
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

// use crate::schema::users;
// use crate::AppState;
// use serde::{Deserialize, Serialize};

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
    let res = response::UserResponse::from(user, token);
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

    let res = response::UserResponse::from(user, token);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn me(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let head = req.head();
    let extensions = head.extensions();
    let user = extensions.get::<User>();

    if let Some(user) = user {
        let user = response::UserResponse::from(user.clone(), user.generate_token());
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::Ok().json({}))
    }
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::Update>,
) -> Result<HttpResponse, HttpResponse> {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invalid auth user").clone();
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
    let res = response::UserResponse::from(user, token.to_string());

    Ok(HttpResponse::Ok().json(res))
}
