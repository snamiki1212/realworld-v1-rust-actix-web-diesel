use uuid::Uuid;

use crate::{
    appv2::features::{
        article::entities::{Article, FetchBySlugAndAuthorId},
        profile::{
            entities::Profile,
            services::{conver_user_to_profile, ConverUserToProfile},
        },
        user::entities::User,
    },
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

    pub fn fetch_comments_list(
        &self,
        current_user: &Option<User>,
    ) -> Result<Vec<(Comment, Profile)>, AppError> {
        let conn = &mut self.pool.get()?;

        let comments = {
            use crate::schema::comments;
            // use crate::schema::comments::dsl::*;
            use crate::schema::users;
            use diesel::prelude::*;
            comments::table
                .inner_join(users::table)
                // .filter(comments::article_id.eq(article_id))
                .get_results::<(Comment, User)>(conn)?
        };

        let comments = comments
            .iter()
            .map(|(comment, user)| {
                // TODO: avoid N+1. Write one query to fetch all data somehow.
                let profile =
                    conver_user_to_profile(conn, &ConverUserToProfile { user, current_user });

                // TODO: avoid copy
                (comment.to_owned(), profile)
            })
            .collect::<Vec<(Comment, Profile)>>();

        Ok(comments)
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
