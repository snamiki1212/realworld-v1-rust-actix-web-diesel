use super::{
    requests,
    usecases::{
        CreateArticleUsecaseInput, DeleteArticleUsecaseInput, FetchArticlesUsecaseInput,
        UpdateArticleUsecaseInput,
    },
};
use crate::app::drivers::middlewares::auth;
use crate::app::drivers::middlewares::state::AppState;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpRequest};
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
        .fetch_articles(FetchArticlesUsecaseInput {
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
    let current_user = auth::get_current_user(&req)?;
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);
    state
        .di_container
        .article_usecase
        .fetch_following_articles(current_user, offset, limit)
}

pub async fn show(state: web::Data<AppState>, path: web::Path<ArticleTitleSlug>) -> ApiResponse {
    let article_title_slug = path.into_inner();
    state
        .di_container
        .article_usecase
        .fetch_article_by_slug(article_title_slug)
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
        .create_article(CreateArticleUsecaseInput {
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
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    let title = form.article.title.clone();
    let description = form.article.description.clone();
    let body = form.article.body.clone();
    state
        .di_container
        .article_usecase
        .update_article(UpdateArticleUsecaseInput {
            current_user,
            article_title_slug,
            title,
            description,
            body,
        })
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
        .delete_article(DeleteArticleUsecaseInput {
            author_id: current_user.id,
            slug: article_title_slug,
        })
}
