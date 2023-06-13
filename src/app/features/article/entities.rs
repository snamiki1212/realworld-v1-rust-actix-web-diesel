use crate::app::features::favorite::entities::Favorite;
use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::schema::articles;
use crate::utils::converter;
use chrono::NaiveDateTime;
use diesel::dsl::Eq;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations, Clone)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = articles)]
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

type WithAuthorId<T> = Eq<articles::author_id, T>;
type WithSlug<T> = Eq<articles::slug, T>;
type WithId<T> = Eq<articles::id, T>;

impl Article {
    fn with_author_id(author_id: &Uuid) -> WithAuthorId<&Uuid> {
        articles::author_id.eq(author_id)
    }

    fn with_slug(slug: &str) -> WithSlug<&str> {
        articles::slug.eq(slug)
    }

    fn with_id(id: &Uuid) -> WithId<&Uuid> {
        articles::id.eq(id)
    }
}

impl Article {
    pub fn create(conn: &mut PgConnection, record: &CreateArticle) -> Result<Self, AppError> {
        let article = diesel::insert_into(articles::table)
            .values(record)
            .get_result::<Article>(conn)?;

        Ok(article)
    }

    pub fn update(
        conn: &mut PgConnection,
        article_title_slug: &str,
        author_id: &Uuid,
        record: &UpdateArticle,
    ) -> Result<Self, AppError> {
        let t = articles::table
            .filter(Self::with_slug(article_title_slug))
            .filter(Self::with_author_id(author_id));
        let article = diesel::update(t).set(record).get_result::<Article>(conn)?;
        Ok(article)
    }

    pub fn convert_title_to_slug(title: &str) -> String {
        converter::to_kebab(title)
    }

    pub fn fetch_by_slug_and_author_id(
        conn: &mut PgConnection,
        params: &FetchBySlugAndAuthorId,
    ) -> Result<Self, AppError> {
        let t = articles::table
            .filter(Self::with_slug(&params.slug))
            .filter(Self::with_author_id(&params.author_id));
        let item = t.first::<Self>(conn)?;
        Ok(item)
    }

    pub fn fetch_by_slug_with_author(
        conn: &mut PgConnection,
        slug: &str,
    ) -> Result<(Self, User), AppError> {
        use crate::schema::users;
        let t = articles::table
            .inner_join(users::table)
            .filter(Self::with_slug(slug));
        let result = t.get_result::<(Self, User)>(conn)?;
        Ok(result)
    }

    pub fn fetch_ids_by_author_name(
        conn: &mut PgConnection,
        name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::users;
        let t = users::table
            .inner_join(articles::table)
            .filter(User::with_username(name))
            .select(articles::id);
        let ids = t.load::<Uuid>(conn)?;
        Ok(ids)
    }

    pub fn find_with_author(conn: &mut PgConnection, id: &Uuid) -> Result<(Self, User), AppError> {
        use crate::schema::users;
        let t = articles::table
            .inner_join(users::table)
            .filter(Self::with_id(id));
        let result = t.get_result::<(Article, User)>(conn)?;
        Ok(result)
    }

    pub fn delete(conn: &mut PgConnection, params: &DeleteArticle) -> Result<(), AppError> {
        let t = articles::table
            .filter(Self::with_slug(&params.slug))
            .filter(Self::with_author_id(&params.author_id));
        diesel::delete(t).execute(conn)?;
        // NOTE: references tag rows are deleted automatically by DELETE CASCADE

        Ok(())
    }
}

impl Article {
    pub fn is_favorited_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: &Uuid,
    ) -> Result<bool, AppError> {
        use crate::schema::favorites;
        let t = favorites::table
            .select(diesel::dsl::count(favorites::id))
            .filter(Favorite::with_article_id(&self.id))
            .filter(Favorite::with_user_id(user_id));
        let count = t.first::<i64>(conn)?;
        Ok(count >= 1)
    }

    pub fn fetch_favorites_count(&self, conn: &mut PgConnection) -> Result<i64, AppError> {
        use crate::schema::favorites;
        let t = favorites::table
            .filter(Favorite::with_article_id(&self.id))
            .select(diesel::dsl::count(favorites::created_at));
        let favorites_count = t.first::<i64>(conn)?;
        Ok(favorites_count)
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = articles)]
pub struct CreateArticle {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = articles)]
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
