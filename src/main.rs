#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
mod app;
mod constants;
mod error;
mod middleware;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start conduit server...");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        middleware::state::AppState { pool }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(middleware::cors::cors())
            .wrap(middleware::auth::Authentication)
            .configure(routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}
