use super::entities::{CreateFavorite, DeleteFavorite, Favorite};
use crate::app::features::article::entities::{Article, FetchBySlugAndAuthorId};
use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

pub trait FavoriteRepository: Send + Sync + 'static {
    fn favorite_article(&self, user: User, article_title_slug: String)
        -> Result<Article, AppError>;
    fn unfavorite_article(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<Article, AppError>;
}

#[derive(Clone)]
pub struct FavoriteRepositoryImpl {
    pool: DbPool,
}

impl FavoriteRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
impl FavoriteRepository for FavoriteRepositoryImpl {
    fn favorite_article(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<Article, AppError> {
        let conn = &mut self.pool.get()?;

        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug,
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

    fn unfavorite_article(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<Article, AppError> {
        let conn = &mut self.pool.get()?;
        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug,
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
