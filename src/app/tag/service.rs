// #[macro_use]
// extern crate diesel;
// extern crate dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::app::tag::model::{NewTag, Tag};
use crate::schema;

pub fn create_tag<'a>(conn: &PgConnection, name: &'a str) -> Tag {
    use schema::tags;
    let new_tag = NewTag { name: name };
    diesel::insert_into(tags::table)
        .values(&new_tag)
        .get_result(conn)
        .expect("Error saving new post")
}
