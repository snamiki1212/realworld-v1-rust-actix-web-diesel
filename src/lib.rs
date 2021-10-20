#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewTag, Tag};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_tag<'a>(conn: &PgConnection, name: &'a str) -> Tag {
    use schema::tags;
    let new_tag = NewTag { name: name };
    diesel::insert_into(tags::table)
        .values(&new_tag)
        .get_result(conn)
        .expect("Error saving new post")
}
