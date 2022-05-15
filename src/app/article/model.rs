use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::articles;
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
        author_id: &Uuid,
        record: &UpdateArticle,
    ) -> Result<Self, AppError> {
        let article = diesel::update(
            articles::table
                .filter(articles::slug.eq(article_title_slug))
                .filter(articles::author_id.eq_all(author_id)),
        )
        .set(record)
        .get_result::<Article>(conn)?;
        Ok(article)
    }

    pub fn convert_title_to_slug(title: &str) -> String {
        converter::to_kebab(title)
    }

    pub fn fetch_by_slug_and_author_id(
        conn: &PgConnection,
        params: &FetchBySlugAndAuthorId,
    ) -> Result<Self, AppError> {
        let item = articles::table
            .filter(articles::slug.eq_all(params.slug.to_owned()))
            .filter(articles::author_id.eq_all(params.author_id))
            .first::<Self>(conn)?;
        Ok(item)
    }

    pub fn fetch_by_slug_with_author(
        conn: &PgConnection,
        slug: &str,
    ) -> Result<(Self, User), AppError> {
        use crate::schema::users;
        let result = articles::table
            .inner_join(users::table)
            .filter(articles::slug.eq(slug))
            .get_result::<(Self, User)>(conn)?;
        Ok(result)
    }

    pub fn fetch_ids_by_author_name(
        conn: &PgConnection,
        name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::users;
        let ids = users::table
            .inner_join(articles::table)
            .filter(users::username.eq(name))
            .select(articles::id)
            .load::<Uuid>(conn)?;
        Ok(ids)
    }

    pub fn find_with_author(conn: &PgConnection, id: &Uuid) -> Result<(Self, User), AppError> {
        use crate::schema::users;
        let result = articles::table
            .inner_join(users::table)
            .filter(articles::id.eq(id))
            .get_result::<(Article, User)>(conn)?;
        Ok(result)
    }

    pub fn delete(conn: &PgConnection, params: &DeleteArticle) -> Result<(), AppError> {
        let _ = diesel::delete(
            articles::table
                .filter(articles::slug.eq(&params.slug))
                .filter(articles::author_id.eq(params.author_id)),
        )
        .execute(conn)?;
        // NOTE: references tag rows are deleted automatically by DELETE CASCADE

        Ok(())
    }
}

impl Article {
    pub fn is_favorited_by_user_id(
        &self,
        conn: &PgConnection,
        user_id: &Uuid,
    ) -> Result<bool, AppError> {
        use crate::schema::favorites;
        let count = favorites::table
            .select(diesel::dsl::count(favorites::id))
            .filter(favorites::article_id.eq_all(self.id))
            .filter(favorites::user_id.eq_all(user_id))
            .first::<i64>(conn)?;

        Ok(count >= 1)
    }

    pub fn fetch_favorites_count(&self, conn: &PgConnection) -> Result<i64, AppError> {
        use crate::schema::favorites;
        let favorites_count = favorites::table
            .filter(favorites::article_id.eq_all(self.id))
            .select(diesel::dsl::count(favorites::created_at))
            .first::<i64>(conn)?;
        Ok(favorites_count)
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

pub struct DeleteArticle {
    pub slug: String,
    pub author_id: Uuid,
}
