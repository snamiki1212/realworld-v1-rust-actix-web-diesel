use super::{
    response,
    service::{self, UnfavoriteService},
};
use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::uuid;
use actix_web::{web, HttpRequest, HttpResponse};

type ArticleIdSlug = String;

pub async fn favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    let article_id = uuid::parse(&article_id)?;

    // TODO: validate article_id

    let (article, profile, favorite_info, tags_list) = service::favorite(
        &conn,
        &service::FavoriteService {
            me: auth_user,
            article_id: article_id,
        },
    )?;
    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tags_list));

    Ok(HttpResponse::Ok().json(res))
}

pub async fn unfavorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    let article_id = uuid::parse(&article_id)?;

    // TODO: validate article_id

    let (article, profile, favorite_info, tags_list) = service::unfavorite(
        &conn,
        &UnfavoriteService {
            me: auth_user,
            article_id: article_id,
        },
    )?;
    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tags_list));
    Ok(HttpResponse::Ok().json(res))
}
