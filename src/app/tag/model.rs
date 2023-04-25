use crate::app::article::model::Article;
use crate::error::AppError;
use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::Insertable;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl Tag {
    pub fn fetch_by_article_id(
        conn: &mut PgConnection,
        article_id: Uuid,
    ) -> Result<Vec<Self>, AppError> {
        let list = tags::table
            .filter(tags::article_id.eq(article_id))
            .get_results::<Self>(conn)?;
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
        let ids = tags::table
            .filter(tags::name.eq(tag_name))
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
