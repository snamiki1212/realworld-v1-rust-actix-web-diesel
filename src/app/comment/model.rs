use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Serialize, Queryable, Associations, Debug, Clone)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comment {
    pub fn create(conn: &mut PgConnection, record: &CreateComment) -> Result<Self, AppError> {
        let new_comment = diesel::insert_into(comments::table)
            .values(record)
            .get_result::<Comment>(conn)?;
        Ok(new_comment)
    }

    pub fn delete(conn: &mut PgConnection, params: &DeleteComment) -> Result<(), AppError> {
        let _ = diesel::delete(comments::table)
            .filter(comments::id.eq(params.comment_id))
            .filter(comments::author_id.eq(params.author_id))
            .filter(comments::article_id.eq(params.article_id))
            .execute(conn)?;
        Ok(())
    }
}

pub struct DeleteComment {
    pub comment_id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
}

#[derive(Insertable, Clone)]
#[table_name = "comments"]
pub struct CreateComment {
    pub body: String,
    pub author_id: Uuid,
    pub article_id: Uuid,
}
