use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::favorites;
use chrono::NaiveDateTime;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Clone, Debug)]
#[belongs_to(Article, foreign_key = "article_id")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "favorites"]
pub struct Favorite {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Favorite {
    pub fn favorite(conn: &PgConnection, record: &FavorteAction) -> Result<usize, AppError> {
        let item = diesel::insert_into(favorites::table)
            .values(record)
            .execute(conn)?;
        Ok(item)
    }

    pub fn unfavorite(conn: &PgConnection, params: &UnfavoriteAction) -> Result<usize, AppError> {
        let item = diesel::delete(favorites::table)
            .filter(favorites::user_id.eq_all(params.user_id))
            .filter(favorites::article_id.eq_all(params.article_id))
            .execute(conn)?;
        Ok(item)
    }
}

#[derive(Insertable)]
#[table_name = "favorites"]
pub struct FavorteAction {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

pub struct UnfavoriteAction {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

#[derive(Clone)]
pub struct FavoriteInfo {
    pub is_favorited: bool,
    pub favorites_count: i64,
}
