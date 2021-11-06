use super::model::Article;
use super::service;
use super::{request, response};
use crate::middleware::auth;
use crate::middleware::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

type ArticleIdSlug = Uuid;

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
    req: HttpRequest,
    params: web::Query<ArticlesListQueryParameter>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);

    let (articles_list, articles_count) = service::fetch_articles_list(
        &conn,
        service::FetchArticlesList {
            tag: params.tag.clone(),
            author: params.author.clone(),
            favorited: params.favorited.clone(),
            offset: offset,
            limit: limit,
            me: auth_user,
        },
    )?;

    let res = response::MultipleArticlesResponse::from((articles_list, articles_count));
    Ok(HttpResponse::Ok().json(res))
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
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);
    let (articles_list, articles_count) = service::fetch_following_articles(
        &conn,
        &service::FetchFollowedArticlesSerivce {
            me: auth_user,
            offset: offset,
            limit: limit,
        },
    )?;

    let res = response::MultipleArticlesResponse::from((articles_list, articles_count));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    let (article, profile, favorite_info, tags_list) = service::fetch_article(
        &conn,
        &service::FetchArticle {
            article_id: article_id,
            me: auth_user,
        },
    )?;

    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tags_list));

    Ok(HttpResponse::Ok().json(res))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::CreateArticleRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let (article, profile, favorite_info, tag_list) = service::create(
        &conn,
        &service::CreateArticleSerivce {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: Article::convert_title_to_slug(&form.article.title),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
            tag_list: form.article.tag_list.to_owned(),
            me: auth_user,
        },
    )?;
    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tag_list));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::UpdateArticleRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    // TODO: validation deletable auth_user.id == article.author_id ?
    let article_slug = &form
        .article
        .title
        .as_ref()
        .map(|_title| Article::convert_title_to_slug(_title));

    // TODO: validation: slug is not empty

    let (article, profile, favorite_info, tag_list) = service::update_article(
        &conn,
        &service::UpdateArticleService {
            me: auth_user,
            article_id,
            slug: article_slug.to_owned(),
            title: form.article.title.clone(),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
    )?;

    let res = response::SingleArticleResponse::from((article, profile, favorite_info, tag_list));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: web::Data<AppState>,
    // req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> Result<HttpResponse, HttpResponse> {
    // let auth_user = auth::access_auth_user(&req)?;
    let conn = state.get_conn()?;
    let article_id = path.into_inner();
    let _ = service::delete_article(&conn, article_id)?;
    Ok(HttpResponse::Ok().json({}))
}
