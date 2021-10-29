use super::model::{Comment, CreateComment};
use super::{request, response, service};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

type ArticleIdSlug = String;

pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments index")
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid url:article id is invalid.");

    let (comment, profile) = service::create(
        &conn,
        &service::CreateCommentService {
            body: form.comment.body.to_owned(),
            article_id: article_id,
            author: auth_user,
        },
    );

    let res = response::SingleCommentResponse::from(comment, profile);
    HttpResponse::Ok().json(res)
}

pub async fn delete() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("comments delete")
}
