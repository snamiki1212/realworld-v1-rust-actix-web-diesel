use super::model::{Comment, CreateComment, DeleteComment};
use crate::app::article::model::{Article, FetchBySlugAndAuthorId};
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::profile::services::{conver_user_to_profile, ConverUserToProfile};
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct CreateCommentService {
    pub body: String,
    pub article_title_slug: String,
    pub author: User,
}
pub fn create(
    conn: &mut PgConnection,
    params: &CreateCommentService,
) -> Result<(Comment, Profile), AppError> {
    let CreateCommentService {
        body,
        article_title_slug,
        author,
    } = params;
    let article = Article::fetch_by_slug_and_author_id(
        conn,
        &FetchBySlugAndAuthorId {
            slug: article_title_slug.to_owned(),
            author_id: author.id,
        },
    )?;
    let comment = Comment::create(
        conn,
        &CreateComment {
            body: body.to_string(),
            author_id: author.id,
            article_id: article.id.to_owned(),
        },
    )?;
    let profile = author.fetch_profile(conn, &author.id)?;
    Ok((comment, profile))
}

pub fn fetch_comments_list(
    conn: &mut PgConnection,
    current_user: &Option<User>,
) -> Result<Vec<(Comment, Profile)>, AppError> {
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
            let profile = conver_user_to_profile(conn, &ConverUserToProfile { user, current_user });

            // TODO: avoid copy
            (comment.to_owned(), profile)
        })
        .collect::<Vec<(Comment, Profile)>>();

    Ok(comments)
}

pub struct DeleteCommentService {
    pub article_title_slug: String,
    pub author_id: Uuid,
    pub comment_id: Uuid,
}

pub fn delete_comment(
    conn: &mut PgConnection,
    params: &DeleteCommentService,
) -> Result<(), AppError> {
    let article = Article::fetch_by_slug_and_author_id(
        conn,
        &FetchBySlugAndAuthorId {
            slug: params.article_title_slug.to_owned(),
            author_id: params.author_id,
        },
    )?;
    Comment::delete(
        conn,
        &DeleteComment {
            comment_id: params.comment_id,
            article_id: article.id,
            author_id: params.author_id,
        },
    )?;
    Ok(())
}
