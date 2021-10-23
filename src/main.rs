#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
mod app;
mod constants;
mod middleware;
mod routes;
mod schema;
mod utils;

pub struct AppState {
    pub pool: utils::db::DbPool,
    // pub request: std::any,
    pub auth_user: Option<crate::app::user::model::User>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let pool = utils::db::establish_connection();
        App::new()
            .wrap(Logger::default())
            .data(AppState {
                pool: pool,
                auth_user: None,
            })
            .wrap(middleware::auth::Authentication)
            .service(web::scope("").configure(routes::api)) // TODO: call configure without emptpy scope
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
