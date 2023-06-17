use crate::app::features::article::entities::Article;
use crate::app::features::user::entities::User;
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

impl Comment {
    fn with_id(id: &Uuid) -> WithId<&Uuid> {
        comments::id.eq(id)
    }
    fn with_author(author_id: &Uuid) -> WithAuthor<&Uuid> {
        comments::author_id.eq(author_id)
    }
}

impl Comment {
    pub fn create(conn: &mut PgConnection, record: &CreateComment) -> Result<Self, AppError> {
        let new_comment = diesel::insert_into(comments::table)
            .values(record)
            .get_result::<Comment>(conn)?;
        Ok(new_comment)
    }

    pub fn delete(
        conn: &mut PgConnection,
        (comment_id, author_id, slug): (&Uuid, &Uuid, &str),
    ) -> Result<(), AppError> {
        let subquery = {
            use crate::schema::articles;
            articles::table
                .filter(articles::slug.eq(slug))
                .filter(articles::author_id.eq(author_id))
                .select(articles::id)
        };

        let query = comments::table
            .filter(Self::with_id(comment_id))
            .filter(Self::with_author(author_id))
            .filter(comments::article_id.eq_any(subquery));

        diesel::delete(query).execute(conn)?;
        Ok(())
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = comments)]
pub struct CreateComment {
    pub body: String,
    pub author_id: Uuid,
    pub article_id: Uuid,
}
