use super::request;
use crate::app::drivers::middlewares::auth;
use crate::app::drivers::middlewares::state::AppState;
use crate::utils::api::ApiResponse;
use crate::utils::uuid;
use actix_web::{web, HttpRequest};

type ArticleIdSlug = String;
type CommentIdSlug = String;

pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> ApiResponse {
    let current_user = auth::get_current_user(&req).ok();
    state
        .di_container
        .comment_usecase
        .fetch_comments(&current_user)
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    let body = form.comment.body.to_owned();
    state
        .di_container
        .comment_usecase
        .create_comment(body, article_title_slug, current_user)
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(ArticleIdSlug, CommentIdSlug)>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let (article_title_slug, comment_id) = path.into_inner();
    let comment_id = uuid::parse(&comment_id)?;
    state.di_container.comment_usecase.delete_comment(
        &article_title_slug,
        comment_id,
        current_user.id,
    )
}
