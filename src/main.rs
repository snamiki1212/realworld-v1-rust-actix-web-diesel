#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
mod appv2;
mod constants;
mod error;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start conduit server...");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        use appv2::drivers::middlewares::state::AppState;
        AppState::new(pool)
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(appv2::drivers::middlewares::cors::cors())
            .wrap(appv2::drivers::middlewares::auth::Authentication)
            .configure(appv2::drivers::routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}
