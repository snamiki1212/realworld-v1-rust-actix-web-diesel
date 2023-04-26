use crate::app::article::model::Article;
use crate::error::AppError;
use crate::schema::tags;
use chrono::NaiveDateTime;
// use diesel::dsl::Eq;
// use diesel::dsl::{AsSelect, SqlTypeOf};
// use diesel::expression::{AsExpression, Expression};
// use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::sql_function;
use diesel::sql_types;
use diesel::Insertable;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type AllColumns = (
    tags::id,
    tags::article_id,
    tags::name,
    tags::created_at,
    tags::updated_at,
);

pub const ALL_COLUMNS: AllColumns = (
    tags::id,
    tags::article_id,
    tags::name,
    tags::created_at,
    tags::updated_at,
);

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, Associations)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

sql_function!(fn canon_name(x: sql_types::Text) -> sql_types::Text);
sql_function!(fn canon_id(x: sql_types::Uuid) -> sql_types::Uuid);

// General
// type SqlType = SqlTypeOf<AsSelect<Tag, Pg>>;
// type BoxedQuery<'a> = tags::BoxedQuery<'a, Pg, SqlType>;

// Tags
type CanonName<T> = canon_name::HelperType<T>;
type CanonArticleId<T> = canon_id::HelperType<T>;
type All = diesel::dsl::Select<tags::table, AllColumns>;
type WithName<'a> = diesel::dsl::Eq<CanonName<tags::name>, CanonName<&'a str>>;
type ByName<'a> = diesel::dsl::Filter<All, WithName<'a>>;
type WithArticleId<'a> =
    diesel::dsl::Eq<CanonArticleId<tags::article_id>, CanonArticleId<&'a Uuid>>;
type ByArticleId<'a> = diesel::dsl::Filter<All, WithArticleId<'a>>;

impl Tag {
    // pub fn with_name<T>(name: T) -> WithName<'static>
    // where
    //     T: AsExpression<sql_types::Text>,
    // {
    //     canon_name(tags::name).eq(canon_name(name))
    // }
    // fn by_article_id<'a, T>(article_id: T) -> BoxedQuery<'a>
    // where
    //     T: AsExpression<sql_types::Uuid>,
    //     T::Expression: BoxableExpression<tags::table, Pg>,
    // {
    //     tags::table.filter(with_article_id(article_id))
    // }

    // fn select_article_ids() -> BoxedQuery<'static> {
    //     // tags::table.select(Tag::as_select()).into_boxed()
    //     tags::table.select(tags::article_id).into_boxed()
    // }

    // fn by_name<'a, T>(name: T) -> BoxedQuery<'a>
    // where
    //     T: AsExpression<sql_types::Text>,
    //     T::Expression: BoxableExpression<tags::table, Pg>,
    // {
    //     tags::table.filter(with_name(name))
    //     // Self::select_article_ids().filter(with_name(name))
    // }

    pub fn all() -> All {
        tags::table.select(ALL_COLUMNS)
    }

    pub fn with_name(name: &str) -> WithName<'_> {
        canon_name(tags::name).eq(canon_name(name))
    }

    pub fn by_name(name: &str) -> ByName<'_> {
        Self::all().filter(Self::with_name(name))
    }

    pub fn with_article_id(article_id: &Uuid) -> WithArticleId<'_> {
        canon_id(tags::article_id).eq(canon_id(article_id))
    }

    pub fn by_article_id(article_id: &Uuid) -> ByArticleId<'_> {
        Self::all().filter(Self::with_article_id(article_id))
    }

    pub fn fetch_by_article_id(
        conn: &mut PgConnection,
        article_id: Uuid,
    ) -> Result<Vec<Self>, AppError> {
        let list = Self::by_article_id(&article_id).get_results::<Self>(conn)?;
        Ok(list)
    }

    pub fn fetch(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = tags::table.load::<Self>(conn)?;
        Ok(list)
    }

    pub fn fetch_ids_by_name(
        conn: &mut PgConnection,
        tag_name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let ids = Self::by_name(tag_name)
            .select(tags::article_id)
            .load::<Uuid>(conn)?;
        Ok(ids)
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
#[table_name = "tags"]
pub struct CreateTag<'a> {
    pub name: &'a str,
    pub article_id: &'a Uuid,
}
