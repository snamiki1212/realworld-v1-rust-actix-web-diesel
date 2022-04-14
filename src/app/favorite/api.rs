use super::{
    response::SingleArticleResponse,
    service::{self, UnfavoriteService},
};
use crate::middleware::state::AppState;
use crate::{error::AppError, middleware::auth};
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleIdSlug = String;

pub async fn favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, AppError> {
    let current_user = auth::get_current_user(&req)?;
    let conn = state.get_conn()?;
    let article_title_slug = path.into_inner();
    let (article, profile, favorite_info, tags_list) = service::favorite(
        &conn,
        &service::FavoriteService {
            current_user,
            article_title_slug,
        },
    )?;
    let res = SingleArticleResponse::from((article, profile, favorite_info, tags_list));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn unfavorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, AppError> {
    let current_user = auth::get_current_user(&req)?;
    let conn = state.get_conn()?;
    let article_title_slug = path.into_inner();
    let (article, profile, favorite_info, tags_list) = service::unfavorite(
        &conn,
        &UnfavoriteService {
            current_user,
            article_title_slug,
        },
    )?;
    let res = SingleArticleResponse::from((article, profile, favorite_info, tags_list));
    Ok(HttpResponse::Ok().json(res))
}
