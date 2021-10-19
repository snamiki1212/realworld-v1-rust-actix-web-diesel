use actix_web::{web, App, HttpServer};
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::scope("/tags").service(api::tags::index))
                .service(
                    web::scope("/users")
                        .service(api::auth::signin)
                        .service(api::auth::signup),
                )
                .service(
                    web::scope("/user")
                        .service(api::users::me)
                        .service(api::users::update),
                )
                .service(
                    web::scope("/profiles")
                        .service(api::profiles::show)
                        .service(api::profiles::follow)
                        .service(api::profiles::unfollow),
                )
                .service(
                    web::scope("/articles/{id}/comments")
                        .service(api::articles::comments::index)
                        .service(api::articles::comments::create)
                        .service(api::articles::comments::delete),
                )
                .service(
                    web::scope("/articles/{id}/favorites")
                        .service(api::articles::favorites::favorite)
                        .service(api::articles::favorites::unfavorite),
                )
                .service(
                    web::scope("/articles")
                        .service(api::articles::index)
                        .service(api::articles::feed)
                        .service(api::articles::show)
                        .service(api::articles::create)
                        .service(api::articles::update)
                        .service(api::articles::delete),
                ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
