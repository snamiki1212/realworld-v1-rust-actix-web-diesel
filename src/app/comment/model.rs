use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable, Associations, Debug, Clone)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Article, foreign_key = "article_id")]
#[table_name = "comments"]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[table_name = "comments"]
pub struct CreateComment {
    pub body: String,
    pub author_id: Uuid,
    pub article_id: Uuid,
}
impl Comment {
    pub fn create(conn: &PgConnection, record: &CreateComment) -> Self {
        use diesel::prelude::*;
        let new_comment = diesel::insert_into(comments::table)
            .values(record)
            .get_result::<Comment>(conn)
            .expect("could not insert comment.");
        new_comment
    }
}
