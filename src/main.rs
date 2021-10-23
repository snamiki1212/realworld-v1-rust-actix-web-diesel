#[macro_use]
extern crate diesel;
mod schema;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
mod app;
mod constants;
mod middleware;
mod routes;
mod utils;

// pub struct AppState {
//     pool: utils::db::DbPool,
//     // pub request: std::any,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = utils::db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .wrap(middleware::auth::SayHi)
            .service(web::scope("").configure(routes::api)) // TODO: call configure without emptpy scope
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
