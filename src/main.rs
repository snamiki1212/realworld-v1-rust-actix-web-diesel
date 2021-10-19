use actix_web::{web, App, HttpServer};
mod articles;
mod auth;
mod profiles;
mod tags;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::scope("/tags").service(tags::index))
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
                    web::scope("/profiles")
                        .service(profiles::show)
                        .service(profiles::follow)
                        .service(profiles::unfollow),
                )
                .service(
                    web::scope("/articles/{id}/comments")
                        .service(articles::comments::index)
                        .service(articles::comments::create)
                        .service(articles::comments::delete),
                )
                .service(
                    web::scope("/articles/{id}/favorites")
                        .service(articles::favorites::favorite)
                        .service(articles::favorites::unfavorite),
                )
                .service(
                    web::scope("/articles")
                        .service(articles::index)
                        .service(articles::feed)
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
