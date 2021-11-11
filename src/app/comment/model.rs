use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Serialize, Queryable, Associations, Debug, Clone)]
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
    pub fn create(conn: &PgConnection, record: &CreateComment) -> Result<Self, AppError> {
        use diesel::prelude::*;
        let new_comment = diesel::insert_into(comments::table)
            .values(record)
            .get_result::<Comment>(conn)?;
        Ok(new_comment)
    }

    pub fn delete(conn: &PgConnection, params: &DeleteCommentAction) -> Result<(), AppError> {
        use crate::schema::comments;
        use crate::schema::comments::dsl::*;
        use diesel::prelude::*;
        let _ = diesel::delete(comments)
            .filter(comments::id.eq(params.comment_id))
            .filter(comments::author_id.eq(params.author_id))
            .filter(comments::article_id.eq(params.article_id))
            .execute(conn)?;
        Ok(())
    }
}

pub struct DeleteCommentAction {
    pub comment_id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
}
