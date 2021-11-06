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
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let list = service::fetch_comments_list(&conn, &auth_user);
    let res = response::MultipleCommentsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid url:article id is invalid.");

    // TODO: Validate this article of article_id is written by auth_user

    let (comment, profile) = service::create(
        &conn,
        &service::CreateCommentService {
            body: form.comment.body.to_owned(),
            article_id: article_id,
            author: auth_user,
        },
    );

    let res = response::SingleCommentResponse::from((comment, profile));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(ArticleIdSlug, CommentIdSlug)>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (article_id, comment_id) = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid url:article id is invalid.");
    let comment_id = Uuid::parse_str(&comment_id).expect("invalid url:comment id is invalid.");
    // TODO: Validate article exists
    // TODO: Validate comment is written by auth_user

    Comment::delete(&conn, &comment_id);

    Ok(HttpResponse::Ok().json("Ok"))
}
