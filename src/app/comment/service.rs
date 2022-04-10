use super::model::{Comment, CreateComment, DeleteCommentAction};
use crate::app::article::model::{Article, FetchBySlugAndAuthorId};
use crate::app::profile::model::Profile;
use crate::app::profile::service::{
    conver_user_to_profile, fetch_profile_by_id, ConverUserToProfile, FetchProfileById,
};
use crate::app::user::model::User;
use crate::error::AppError;
// use crate::schema::follows;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct CreateCommentService {
    pub body: String,
    pub article_title_slug: String,
    pub author: User,
}
pub fn create(
    conn: &PgConnection,
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
    let profile = fetch_profile_by_id(
        conn,
        &FetchProfileById {
            id: author.id,
            user: author.to_owned(),
        },
    )?;
    Ok((comment, profile))
}

pub fn fetch_comments_list(
    conn: &PgConnection,
    me: &Option<User>,
) -> Result<Vec<(Comment, Profile)>, AppError> {
    use crate::schema::comments;
    use crate::schema::comments::dsl::*;
    use crate::schema::users;
    use diesel::prelude::*;
    let _comments = comments
        .inner_join(users::table)
        .filter(comments::article_id.eq(article_id))
        .get_results::<(Comment, User)>(conn)?;

    let _comments = _comments
        .iter()
        .map(|(_comment, _user)| {
            // TODO: avoid N+1. Write one query to fetch all data somehow.
            let profile = conver_user_to_profile(conn, &ConverUserToProfile { user: _user, me });
            (_comment.to_owned(), profile)
        })
        .collect::<Vec<(Comment, Profile)>>();

    Ok(_comments)
}

pub struct DeleteCommentService {
    pub article_title_slug: String,
    pub author_id: Uuid,
    pub comment_id: Uuid,
}
pub fn delete_comment(conn: &PgConnection, params: &DeleteCommentService) -> Result<(), AppError> {
    let article = Article::fetch_by_slug_and_author_id(
        conn,
        &FetchBySlugAndAuthorId {
            slug: params.article_title_slug.to_owned(),
            author_id: params.author_id,
        },
    )?;
    let _ = Comment::delete(
        conn,
        &DeleteCommentAction {
            comment_id: params.comment_id,
            article_id: article.id,
            author_id: params.author_id,
        },
    )?;
    Ok(())
}
