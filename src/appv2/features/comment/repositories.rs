use uuid::Uuid;

use crate::{
    appv2::features::article::entities::{Article, FetchBySlugAndAuthorId},
    error::AppError,
    utils::db::DbPool,
};

use super::entities::{Comment, DeleteComment};

#[derive(Clone)]
pub struct CommentRepository {
    pool: DbPool,
}

impl CommentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn delete(
        &self,
        article_title_slug: &str,
        comment_id: Uuid,
        author_id: Uuid,
    ) -> Result<(), AppError> {
        let conn = &mut self.pool.get()?;
        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug.to_owned(),
                author_id,
            },
        )?;
        Comment::delete(
            conn,
            &DeleteComment {
                comment_id,
                article_id: article.id,
                author_id,
            },
        )?;
        Ok(())
    }

    // pub fn favorite(&self, user: User, article_title_slug: String) -> Result<Article, AppError> {
    //     // let conn = &mut self.pool.get()?;

    //     // let article = Article::fetch_by_slug_and_author_id(
    //     //     conn,
    //     //     &FetchBySlugAndAuthorId {
    //     //         slug: article_title_slug.to_owned(),
    //     //         author_id: user.id,
    //     //     },
    //     // )?;
    //     // Favorite::create(
    //     //     conn,
    //     //     &CreateFavorite {
    //     //         user_id: user.id,
    //     //         article_id: article.id,
    //     //     },
    //     // )?;

    //     // Ok(article)
    // }
}
