use super::model::{Article, NewArticle, UpdateArticle};
use super::service;
use super::{request, response};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
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
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);

    let (articles_list, articles_count) = {
        let articles_list = service::fetch_articles_list(
            &conn,
            service::FetchArticlesList {
                tag: params.tag.clone(),
                author: params.author.clone(),
                favorited: params.favorited.clone(),
                offset: offset,
                limit: limit,
            },
        );
        let articles_count = service::fetch_articles_count(&conn);
        (articles_list, articles_count)
    };

    let res = response::MultipleArticlesResponse::from(articles_list, articles_count);
    HttpResponse::Ok().json(res)
}

pub async fn feed() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("feed of articles")
}

pub async fn show() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("detail_article")
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::CreateArticleRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let (article, tag_list) = service::create(
        &conn,
        &NewArticle {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: Article::convert_title_to_slug(&form.article.title),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
        &form.article.tagList,
    );
    let res = response::SingleArticleResponse::from(article, auth_user.clone(), tag_list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::UpdateArticleRequest>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //
    let article_id = path.into_inner();

    let (article, tag_list) = {
        // TODO: move this logic to service
        // TODO: validation deletable auth_user.id == article.author_id ?
        let new_slug = &form
            .article
            .title
            .as_ref()
            .map(|_title| Article::convert_title_to_slug(_title));
        let article = Article::update(
            &conn,
            &article_id,
            &UpdateArticle {
                slug: new_slug.to_owned(),
                title: form.article.title.clone(),
                description: form.article.description.clone(),
                body: form.article.body.clone(),
            },
        );
        let tag_list = vec![]; // TODO: fetch tag list
        (article, tag_list)
    };

    let res = response::SingleArticleResponse::from(article, auth_user, tag_list);
    HttpResponse::Ok().json(res)
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("couldn't access auth user.");
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    //
    let article_id = path.into_inner();

    {
        // TODO: move this logic into service
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;

        // TODO: validation deletable auth_user.id == article.author_id ?

        diesel::delete(articles.filter(id.eq(article_id)))
            .execute(&conn)
            .expect("couldn't delete article by id.");
        // NOTE: references tag rows are deleted automatically by DELETE CASCADE
    }

    HttpResponse::Ok().json({})
}
