use crate::app;
use actix_web::web;
use actix_web::web::{delete, get, post, put};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/tags").route("", get().to(app::tag::api::index)))
            .service(
                web::scope("/users")
                    .route("/login", post().to(app::user::api::signin))
                    .route("", post().to(app::user::api::signup)),
            )
            .service(
                web::scope("/user")
                    .route("", get().to(app::user::api::me))
                    .route("", put().to(app::user::api::update)),
            )
            .service(
                web::scope("/profiles")
                    .route("/{username}", get().to(app::profile::api::show))
                    .route("/{username}/follow", post().to(app::profile::api::follow))
                    .route(
                        "/{username}/follow",
                        delete().to(app::profile::api::unfollow),
                    ),
            )
            .service(
                web::scope("/articles/{id}/comments")
                    .route("", get().to(app::articles::comments::index))
                    .route("", post().to(app::articles::comments::create))
                    .route("", delete().to(app::articles::comments::delete)),
            )
            .service(
                web::scope("/articles/{id}/favorites")
                    .route("", post().to(app::articles::favorites::favorite))
                    .route("", delete().to(app::articles::favorites::unfavorite)),
            )
            .service(
                web::scope("/articles")
                    .route("", get().to(app::articles::index))
                    .route("/feed", get().to(app::articles::feed))
                    .route("/{id}", get().to(app::articles::show))
                    .route("", post().to(app::articles::create))
                    .route("/{id}", put().to(app::articles::update))
                    .route("", delete().to(app::articles::delete)),
            ),
    );
}
