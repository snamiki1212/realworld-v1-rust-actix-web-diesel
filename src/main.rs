use actix_web::{web, App, HttpServer};
mod articles;
mod auth;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(
                    web::scope("/users")
                        .service(auth::signin)
                        .service(auth::signup),
                )
                .service(
                    web::scope("/user")
                        .service(users::me)
                        .service(users::update),
                )
                .service(
                    web::scope("/articles")
                        .service(articles::index)
                        .service(articles::show)
                        .service(articles::create)
                        .service(articles::update)
                        .service(articles::delete),
                ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
