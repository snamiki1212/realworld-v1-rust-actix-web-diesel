use crate::app::features::article::entities::Article;
use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::schema::favorites;
use chrono::NaiveDateTime;
use diesel::dsl::Eq;
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

type WithUserId<T> = Eq<favorites::user_id, T>;
type WithArticleId<T> = Eq<favorites::article_id, T>;

impl Favorite {
    pub fn with_user_id(user_id: &Uuid) -> WithUserId<&Uuid> {
        favorites::user_id.eq_all(user_id)
    }

    pub fn with_article_id(article_id: &Uuid) -> WithArticleId<&Uuid> {
        favorites::article_id.eq_all(article_id)
    }
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
        let t = favorites::table
            .filter(Self::with_user_id(user_id))
            .filter(Self::with_article_id(article_id));
        let item = diesel::delete(t).execute(conn)?;
        Ok(item)
    }

    pub fn fetch_favorited_article_ids_by_username(
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::users;
        let t = favorites::table
            .inner_join(users::table)
            .filter(User::with_username(username))
            .select(favorites::article_id);
        let ids = t.load::<Uuid>(conn)?;
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
