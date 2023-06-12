use crate::appv2;
use actix_web::web;
use actix_web::web::{delete, get, post, put};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/healthcheck").route(
                "",
                get().to(appv2::features::healthcheck::controllers::index),
            ))
            .service(
                web::scope("/tags").route("", get().to(appv2::features::tag::controllers::index)),
            )
            .service(
                web::scope("/users")
                    .route(
                        "/login",
                        post().to(appv2::features::user::controllers::signin),
                    )
                    .route("", post().to(appv2::features::user::controllers::signup)),
            )
            .service(
                web::scope("/user")
                    .route("", get().to(appv2::features::user::controllers::me))
                    .route("", put().to(appv2::features::user::controllers::update)),
            )
            .service(
                web::scope("/profiles")
                    .route(
                        "/{username}",
                        get().to(appv2::features::profile::controllers::show),
                    )
                    .route(
                        "/{username}/follow",
                        post().to(appv2::features::profile::controllers::follow),
                    )
                    .route(
                        "/{username}/follow",
                        delete().to(appv2::features::profile::controllers::unfollow),
                    ),
            )
            .service(
                web::scope("/articles")
                    .route(
                        "/feed",
                        get().to(appv2::features::article::controllers::feed),
                    )
                    .route("", get().to(appv2::features::article::controllers::index))
                    .route("", post().to(appv2::features::article::controllers::create))
                    .service(
                        web::scope("/{article_title_slug}")
                            .route("", get().to(appv2::features::article::controllers::show))
                            .route("", put().to(appv2::features::article::controllers::update))
                            .route(
                                "",
                                delete().to(appv2::features::article::controllers::delete),
                            )
                            .service(
                                web::scope("/favorite")
                                    .route(
                                        "",
                                        post().to(appv2::features::favorite::controllers::favorite),
                                    )
                                    .route(
                                        "",
                                        delete()
                                            .to(appv2::features::favorite::controllers::unfavorite),
                                    ),
                            )
                            .service(
                                web::scope("/comments")
                                    .route(
                                        "",
                                        get().to(appv2::features::comment::controllers::index),
                                    )
                                    .route(
                                        "",
                                        post().to(appv2::features::comment::controllers::create),
                                    )
                                    .route(
                                        "/{comment_id}",
                                        delete().to(appv2::features::comment::controllers::delete),
                                    ),
                            ),
                    ),
            ),
    );
}
