use super::model::{Article, NewArticle};
use super::{request, response};
use crate::app::user::model::User;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use convert_case::{Case, Casing};

pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("show_articles")
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
    println!("_----");
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invalid auth user").clone();
    // --
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let tag_list = form.article.tagList.clone();
    let article = Article::create(
        &conn,
        &NewArticle {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: form.article.title.to_case(Case::Kebab),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
        tag_list.clone(),
    );

    let tag_list = match tag_list {
        Some(n) => n,
        None => vec![],
    };
    let res = response::SingleArticleResponse::from(article, auth_user, tag_list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("update_article")
}

pub async fn delete() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("delete_article")
}
