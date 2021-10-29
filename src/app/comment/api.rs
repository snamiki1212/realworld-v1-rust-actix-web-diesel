use super::model::{Comment, CreateComment};
use super::{request, response, service};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

type ArticleIdSlug = String;
type CommentIdSlug = String;

pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //

    let list = service::fetch_comments_list(&conn, &auth_user);
    let res = response::MultipleCommentsResponse::from(list);
    HttpResponse::Ok().json(res)
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

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(ArticleIdSlug, CommentIdSlug)>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //
    let (article_id, comment_id) = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid url:article id is invalid.");
    let comment_id = Uuid::parse_str(&comment_id).expect("invalid url:comment id is invalid.");
    // TODO:

    Comment::delete(&conn, &comment_id);

    HttpResponse::Ok().json("OK")
}
