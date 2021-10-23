use crate::app::user::{model::User, request, response};
use crate::AppState;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};

// use crate::schema::users;
// use crate::AppState;
// use serde::{Deserialize, Serialize};

#[post("/login")]
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

#[post("")]
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

#[get("")]
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

#[put("")]
pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users update")
}
