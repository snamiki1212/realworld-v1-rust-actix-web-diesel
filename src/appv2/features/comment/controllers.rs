use super::{
    presenters::{MultipleCommentsResponse, SingleCommentResponse},
    request, service,
};
use crate::appv2::drivers::middlewares::auth;
use crate::appv2::drivers::middlewares::state::AppState;
use crate::utils::api::ApiResponse;
use crate::utils::uuid;
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleIdSlug = String;
type CommentIdSlug = String;

pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> ApiResponse {
    let current_user = auth::get_current_user(&req).ok();
    state
        .di_container
        .comment_usecase
        .fetch_comments_list(&current_user)
    // let conn = &mut state.get_conn()?;
    // let list = service::fetch_comments_list(conn, &current_user)?;
    // let res = MultipleCommentsResponse::from(list);
    // Ok(HttpResponse::Ok().json(res))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    let (comment, profile) = service::create(
        conn,
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
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let (article_title_slug, comment_id) = path.into_inner();
    let comment_id = uuid::parse(&comment_id)?;
    state
        .di_container
        .comment_usecase
        .delete(&article_title_slug, comment_id, current_user.id)
}
