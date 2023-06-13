use crate::app::features::article::entities::Article;
use crate::error::AppError;
use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::dsl::{AsSelect, Eq, Filter, Select};
use diesel::pg::PgConnection;
use diesel::Insertable;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Identifiable,
    Selectable,
    Queryable,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Associations,
    QueryableByName,
)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Tags
type All<DB> = Select<tags::table, AsSelect<Tag, DB>>;
type WithName<T> = Eq<tags::name, T>;
type ByName<T, DB> = Filter<All<DB>, WithName<T>>;
type WithArticleId<T> = Eq<tags::article_id, T>;
type ByArticleId<T, DB> = Filter<All<DB>, WithArticleId<T>>;

impl Tag {
    fn all<DB>() -> All<DB>
    where
        DB: Backend,
    {
        tags::table.select(Tag::as_select())
    }

    fn with_name(name: &str) -> WithName<&str> {
        tags::name.eq(name)
    }

    fn by_name<DB>(name: &str) -> ByName<&str, DB>
    where
        DB: Backend,
    {
        Self::all().filter(Self::with_name(name))
    }

    fn with_article_id(article_id: &Uuid) -> WithArticleId<&Uuid> {
        tags::article_id.eq(article_id)
    }

    fn by_article_id<DB>(article_id: &Uuid) -> ByArticleId<&Uuid, DB>
    where
        DB: Backend,
    {
        Self::all().filter(Self::with_article_id(article_id))
    }

    pub fn fetch_by_article_id(
        conn: &mut PgConnection,
        article_id: &Uuid,
    ) -> Result<Vec<Self>, AppError> {
        let t = Self::by_article_id(article_id);
        let list = t.get_results::<Self>(conn)?;
        Ok(list)
    }

    pub fn fetch(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = tags::table.load::<Self>(conn)?;
        Ok(list)
    }

    pub fn fetch_article_ids_by_name(
        conn: &mut PgConnection,
        tag_name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let t = Self::by_name(tag_name);
        let article_ids = t
            .load::<Tag>(conn)?
            .iter()
            .map(|tag| tag.article_id)
            .collect();
        Ok(article_ids)
    }

    pub fn create_list(
        conn: &mut PgConnection,
        records: Vec<CreateTag>,
    ) -> Result<Vec<Self>, AppError> {
        let tags_list = diesel::insert_into(tags::table)
            .values(records)
            .get_results::<Tag>(conn)?;
        Ok(tags_list)
    }
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct CreateTag<'a> {
    pub name: &'a str,
    pub article_id: &'a Uuid,
}
