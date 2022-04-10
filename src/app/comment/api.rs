use super::{request, response, service};
use crate::error::AppError;
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::uuid;
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleIdSlug = String;
type CommentIdSlug = String;

pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    let auth_user = auth::access_auth_user(&req).ok();
    let conn = state.get_conn()?;
    let list = service::fetch_comments_list(&conn, &auth_user)?;
    let res = response::MultipleCommentsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_title_slug = path.into_inner();
    let (comment, profile) = service::create(
        &conn,
        &service::CreateCommentService {
            body: form.comment.body.to_owned(),
            article_title_slug,
            author: auth_user,
        },
    )?;
    let res = response::SingleCommentResponse::from((comment, profile));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(ArticleIdSlug, CommentIdSlug)>,
) -> Result<HttpResponse, AppError> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let (article_title_slug, comment_id) = path.into_inner();
    let comment_id = uuid::parse(&comment_id)?;
    let _ = service::delete_comment(
        &conn,
        &service::DeleteCommentService {
            article_title_slug,
            comment_id,
            author_id: auth_user.id,
        },
    )?;
    Ok(HttpResponse::Ok().json("Ok"))
}
