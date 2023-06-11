use crate::app::article::model::Article;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::dsl::Eq;
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

type WithId<T> = Eq<comments::id, T>;
type WithAuthor<T> = Eq<comments::author_id, T>;
type WithArticle<T> = Eq<comments::article_id, T>;

impl Comment {
    fn with_id(id: &Uuid) -> WithId<&Uuid> {
        comments::id.eq(id)
    }
    fn with_author(author_id: &Uuid) -> WithAuthor<&Uuid> {
        comments::author_id.eq(author_id)
    }
    fn with_article(article_id: &Uuid) -> WithArticle<&Uuid> {
        comments::article_id.eq(article_id)
    }
}

impl Comment {
    pub fn create(conn: &mut PgConnection, record: &CreateComment) -> Result<Self, AppError> {
        let new_comment = diesel::insert_into(comments::table)
            .values(record)
            .get_result::<Comment>(conn)?;
        Ok(new_comment)
    }

    pub fn delete(conn: &mut PgConnection, params: &DeleteComment) -> Result<(), AppError> {
        let t = comments::table
            .filter(Self::with_id(&params.comment_id))
            .filter(Self::with_author(&params.author_id))
            .filter(Self::with_article(&params.article_id));
        diesel::delete(t).execute(conn)?;
        Ok(())
    }
}

pub struct DeleteComment {
    pub comment_id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = comments)]
pub struct CreateComment {
    pub body: String,
    pub author_id: Uuid,
    pub article_id: Uuid,
}
