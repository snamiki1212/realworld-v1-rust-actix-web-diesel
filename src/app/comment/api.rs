use super::{
    request,
    response::{MultipleCommentsResponse, SingleCommentResponse},
    service,
};
use crate::error::AppError;
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::uuid;
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleIdSlug = String;
type CommentIdSlug = String;

pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req).ok();
    let list = service::fetch_comments_list(&conn, &current_user)?;
    let res = MultipleCommentsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    let (comment, profile) = service::create(
        &conn,
        &service::CreateCommentService {
            body: form.comment.body.to_owned(),
            article_title_slug,
            author: current_user,
        },
    )?;
    let res = SingleCommentResponse::from((comment, profile));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(ArticleIdSlug, CommentIdSlug)>,
) -> Result<HttpResponse, AppError> {
    let conn = state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let (article_title_slug, comment_id) = path.into_inner();
    let comment_id = uuid::parse(&comment_id)?;
    let _ = service::delete_comment(
        &conn,
        &service::DeleteCommentService {
            article_title_slug,
            comment_id,
            author_id: current_user.id,
        },
    )?;
    Ok(HttpResponse::Ok().json("Ok"))
}
