use super::entities::{CreateFavorite, DeleteFavorite, Favorite};
use crate::appv2::features::article::entities::{Article, FetchBySlugAndAuthorId};
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct FavoriteRepository {
    pool: DbPool,
}

impl FavoriteRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn favorite(&self, user: User, article_title_slug: String) -> Result<Article, AppError> {
        let conn = &mut self.pool.get()?;

        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug.to_owned(),
                author_id: user.id,
            },
        )?;
        Favorite::create(
            conn,
            &CreateFavorite {
                user_id: user.id,
                article_id: article.id,
            },
        )?;

        Ok(article)
    }

    pub fn unfavorite(&self, user: User, article_title_slug: String) -> Result<Article, AppError> {
        let conn = &mut self.pool.get()?;
        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug.to_owned(),
                author_id: user.id,
            },
        )?;
        Favorite::delete(
            conn,
            &DeleteFavorite {
                user_id: user.id,
                article_id: article.id,
            },
        )?;
        Ok(article)
    }
}
