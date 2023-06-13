use crate::app;
use actix_web::web;
use actix_web::web::{delete, get, post, put};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/healthcheck")
                    .route("", get().to(app::features::healthcheck::controllers::index)),
            )
            .service(
                web::scope("/tags").route("", get().to(app::features::tag::controllers::index)),
            )
            .service(
                web::scope("/users")
                    .route(
                        "/login",
                        post().to(app::features::user::controllers::signin),
                    )
                    .route("", post().to(app::features::user::controllers::signup)),
            )
            .service(
                web::scope("/user")
                    .route("", get().to(app::features::user::controllers::me))
                    .route("", put().to(app::features::user::controllers::update)),
            )
            .service(
                web::scope("/profiles")
                    .route(
                        "/{username}",
                        get().to(app::features::profile::controllers::show),
                    )
                    .route(
                        "/{username}/follow",
                        post().to(app::features::profile::controllers::follow),
                    )
                    .route(
                        "/{username}/follow",
                        delete().to(app::features::profile::controllers::unfollow),
                    ),
            )
            .service(
                web::scope("/articles")
                    .route("/feed", get().to(app::features::article::controllers::feed))
                    .route("", get().to(app::features::article::controllers::index))
                    .route("", post().to(app::features::article::controllers::create))
                    .service(
                        web::scope("/{article_title_slug}")
                            .route("", get().to(app::features::article::controllers::show))
                            .route("", put().to(app::features::article::controllers::update))
                            .route("", delete().to(app::features::article::controllers::delete))
                            .service(
                                web::scope("/favorite")
                                    .route(
                                        "",
                                        post().to(app::features::favorite::controllers::favorite),
                                    )
                                    .route(
                                        "",
                                        delete()
                                            .to(app::features::favorite::controllers::unfavorite),
                                    ),
                            )
                            .service(
                                web::scope("/comments")
                                    .route("", get().to(app::features::comment::controllers::index))
                                    .route(
                                        "",
                                        post().to(app::features::comment::controllers::create),
                                    )
                                    .route(
                                        "/{comment_id}",
                                        delete().to(app::features::comment::controllers::delete),
                                    ),
                            ),
                    ),
            ),
    );
}
