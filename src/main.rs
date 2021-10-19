use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("")]
async fn show_articles() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("show_articles")
}

#[get("/{id}")]
async fn detail_article() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("detail_article")
}

#[post("")]
async fn create_article() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("create_article")
}

#[put("/{id}")]
async fn update_article() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("update_article")
}

#[delete("")]
async fn delete_article() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("delete_article")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(
                web::scope("/articles")
                    .service(show_articles)
                    .service(detail_article)
                    .service(create_article)
                    .service(update_article)
                    .service(delete_article),
            ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
