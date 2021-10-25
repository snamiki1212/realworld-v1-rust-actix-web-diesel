// use crate::schema::articles::dsl::*;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

use crate::schema::articles;
use diesel::prelude::*;

impl Article {
    pub fn create(conn: &PgConnection, record: &NewArticle) -> Self {
        let article = diesel::insert_into(articles::table)
            .values(record)
            .get_result::<Article>(conn)
            .expect("couldn't insert article");

        article
    }
}

#[derive(Insertable, Clone)]
#[table_name = "articles"]
pub struct NewArticle {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(AsChangeset)]
#[table_name = "articles"]
pub struct UpdateArticle {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
