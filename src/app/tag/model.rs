use crate::app::article::model::Article;
use crate::schema::tags;
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::Insertable;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, Associations)]
#[belongs_to(Article, foreign_key = "article_id")]
#[table_name = "tags"]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Tag {
    pub fn fetch_list_by_article_id(conn: &PgConnection, _article_id: Uuid) -> Vec<Self> {
        use crate::schema::tags;
        use crate::schema::tags::dsl::*;
        use diesel::prelude::*;
        let list = tags
            .filter(tags::article_id.eq(_article_id))
            .get_results::<Self>(conn)
            .expect("could not fetch tags.");
        list
    }

    pub fn list(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        use crate::schema;
        use diesel::prelude::*;
        use schema::tags::dsl::*;
        let list = tags.load::<Tag>(conn);
        list
    }

    // TODO: rename create_list
    pub fn create(conn: &PgConnection, records: Vec<NewTag>) -> Vec<Self> {
        use crate::diesel::RunQueryDsl;
        use crate::schema::tags::dsl::*;
        // TODO: validate record params are valid.
        let tags_list = diesel::insert_into(tags)
            .values(records)
            .get_results::<Tag>(conn)
            .expect("couldn't insert tags.");

        tags_list
    }
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub name: &'a str,
    pub article_id: &'a Uuid,
}
