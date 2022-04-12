use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::articles;
use crate::schema::articles::dsl::*;
use crate::utils::converter;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations, Clone)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "articles"]
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

impl Article {
    pub fn create(conn: &PgConnection, record: &CreateArticle) -> Result<Self, AppError> {
        let article = diesel::insert_into(articles::table)
            .values(record)
            .get_result::<Article>(conn)?;

        Ok(article)
    }

    pub fn update(
        conn: &PgConnection,
        article_title_slug: &str,
        _author_id: &Uuid,
        record: &UpdateArticle,
    ) -> Result<Self, AppError> {
        let article = diesel::update(
            articles
                .filter(articles::slug.eq(article_title_slug))
                .filter(articles::author_id.eq_all(_author_id)),
        )
        .set(record)
        .get_result::<Article>(conn)?;
        Ok(article)
    }

    pub fn convert_title_to_slug(_title: &str) -> String {
        converter::to_kebab(_title)
    }

    pub fn fetch_by_slug_and_author_id(
        conn: &PgConnection,
        params: &FetchBySlugAndAuthorId,
    ) -> Result<Self, AppError> {
        use crate::schema::articles::dsl::*;
        let item = articles
            .filter(slug.eq_all(params.slug.to_owned()))
            .filter(author_id.eq_all(params.author_id))
            .first::<Self>(conn)?;
        Ok(item)
    }

    pub fn delete(conn: &PgConnection, params: &DeleteArticle) -> Result<(), AppError> {
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;

        let _ = diesel::delete(
            articles
                .filter(slug.eq(&params.slug))
                .filter(author_id.eq(params.author_id)),
        )
        .execute(conn)?;
        // NOTE: references tag rows are deleted automatically by DELETE CASCADE

        Ok(())
    }
}

impl Article {
    pub fn is_favorited_by_user_id(&self, conn: &PgConnection, user_id: &Uuid) -> bool {
        use crate::schema::users;
        articles::table
            .select(articles::id)
            .filter(articles::id.eq(self.id))
            .inner_join(users::table.on(users::id.eq(user_id)))
            .load::<Uuid>(conn)
            .is_ok()
    }
}

#[derive(Insertable, Clone)]
#[table_name = "articles"]
pub struct CreateArticle {
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

pub struct FetchBySlugAndAuthorId {
    pub slug: String,
    pub author_id: Uuid,
}

pub struct IsFavoritedBy {
    pub article_id: Uuid,
    pub user_id: Uuid,
}

pub struct DeleteArticle {
    pub slug: String,
    pub author_id: Uuid,
}
