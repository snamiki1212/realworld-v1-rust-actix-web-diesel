use super::{
    entities::{Article, DeleteArticle},
    presenters::{MultipleArticlesResponse, SingleArticleResponse},
    requests, services,
    usecases::{CreateArticleUsecaseInput, DeleteArticleUsercaseInput},
};
use crate::appv2::drivers::middlewares::auth;
use crate::appv2::drivers::middlewares::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

type ArticleTitleSlug = String;

#[derive(Deserialize)]
pub struct ArticlesListQueryParameter {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn index(
    state: web::Data<AppState>,
    params: web::Query<ArticlesListQueryParameter>,
) -> ApiResponse {
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);
    state
        .di_container
        .article_usecase
        .fetch_articles_list(services::FetchArticlesList {
            tag: params.tag.clone(),
            author: params.author.clone(),
            favorited: params.favorited.clone(),
            offset,
            limit,
        })
}

#[derive(Deserialize)]
pub struct FeedQueryParameter {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn feed(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FeedQueryParameter>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);
    let (articles_list, articles_count) = services::fetch_following_articles(
        conn,
        &services::FetchFollowedArticlesSerivce {
            current_user,
            offset,
            limit,
        },
    )?;

    let res = MultipleArticlesResponse::from((articles_list, articles_count));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn show(state: web::Data<AppState>, path: web::Path<ArticleTitleSlug>) -> ApiResponse {
    let article_title_slug = path.into_inner();
    state
        .di_container
        .article_usecase
        .fetch_article_by_slug(&services::FetchArticleBySlug { article_title_slug })
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<requests::CreateArticleRequest>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    state
        .di_container
        .article_usecase
        .create(CreateArticleUsecaseInput {
            title: form.article.title.clone(),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
            tag_name_list: form.article.tag_list.to_owned(),
            current_user,
        })
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleTitleSlug>,
    form: web::Json<requests::UpdateArticleRequest>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    let article_slug = &form
        .article
        .title
        .as_ref()
        .map(|_title| Article::convert_title_to_slug(_title));

    let (article, profile, favorite_info, tag_list) = services::update_article(
        conn,
        &services::UpdateArticleService {
            current_user,
            article_title_slug,
            slug: article_slug.to_owned(),
            title: form.article.title.clone(),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
    )?;

    let res = SingleArticleResponse::from((article, profile, favorite_info, tag_list));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleTitleSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    state
        .di_container
        .article_usecase
        .delete(DeleteArticleUsercaseInput {
            author_id: current_user.id,
            slug: article_title_slug,
        })
}
