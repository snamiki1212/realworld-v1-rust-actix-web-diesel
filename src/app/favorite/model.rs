use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::favorites;
use chrono::NaiveDateTime;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Clone, Debug)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Favorite {
    pub fn create(conn: &mut PgConnection, record: &CreateFavorite) -> Result<usize, AppError> {
        let item = diesel::insert_into(favorites::table)
            .values(record)
            .execute(conn)?;
        Ok(item)
    }

    pub fn delete(
        conn: &mut PgConnection,
        DeleteFavorite {
            user_id,
            article_id,
        }: &DeleteFavorite,
    ) -> Result<usize, AppError> {
        let item = diesel::delete(favorites::table)
            .filter(favorites::user_id.eq_all(user_id))
            .filter(favorites::article_id.eq_all(article_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn fetch_favorited_article_ids_by_username(
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::users;
        let ids = favorites::table
            .inner_join(users::table)
            .filter(users::username.eq(username))
            .select(favorites::article_id)
            .load::<Uuid>(conn)?;
        Ok(ids)
    }
}

#[derive(Insertable)]
#[diesel(table_name = favorites)]
pub struct CreateFavorite {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

pub struct DeleteFavorite {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

#[derive(Clone)]
pub struct FavoriteInfo {
    pub is_favorited: bool,
    pub favorites_count: i64,
}
