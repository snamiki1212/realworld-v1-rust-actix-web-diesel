use super::entities::{Comment, CreateComment};
use crate::{
    app::features::{
        article::entities::{Article, FetchBySlugAndAuthorId},
        profile::entities::Profile,
        user::entities::User,
    },
    error::AppError,
    utils::db::DbPool,
};
use uuid::Uuid;

pub trait CommentRepository: Send + Sync + 'static {
    fn fetch_comments(
        &self,
        current_user: &Option<User>,
    ) -> Result<Vec<(Comment, Profile)>, AppError>;

    fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<(Comment, Profile), AppError>;

    fn delete_comment(
        &self,
        article_title_slug: &str,
        comment_id: Uuid,
        author_id: Uuid,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct CommentRepositoryImpl {
    pool: DbPool,
}

impl CommentRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
impl CommentRepository for CommentRepositoryImpl {
    fn fetch_comments(
        &self,
        current_user: &Option<User>,
    ) -> Result<Vec<(Comment, Profile)>, AppError> {
        let conn = &mut self.pool.get()?;

        let comments = {
            use crate::schema::comments;
            use crate::schema::users;
            use diesel::prelude::*;
            comments::table
                .inner_join(users::table)
                .get_results::<(Comment, User)>(conn)?
        };

        let comments = comments
            .iter()
            .map(|(comment, user)| {
                // TODO: avoid N+1. Write one query to fetch all data somehow.
                let profile = user.to_profile(conn, current_user);

                // TODO: avoid copy
                (comment.to_owned(), profile)
            })
            .collect::<Vec<(Comment, Profile)>>();

        Ok(comments)
    }

    fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<(Comment, Profile), AppError> {
        let conn = &mut self.pool.get()?;

        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug,
                author_id: author.id,
            },
        )?;
        let comment = Comment::create(
            conn,
            &CreateComment {
                body,
                author_id: author.id,
                article_id: article.id.to_owned(),
            },
        )?;
        let profile = author.fetch_profile(conn, &author.id)?;
        Ok((comment, profile))
    }

    fn delete_comment(
        &self,
        article_title_slug: &str,
        comment_id: Uuid,
        author_id: Uuid,
    ) -> Result<(), AppError> {
        let conn = &mut self.pool.get()?;
        let _ = Comment::delete(conn, (&comment_id, &author_id, article_title_slug));
        Ok(())
    }
}
