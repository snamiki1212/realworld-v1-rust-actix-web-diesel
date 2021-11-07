use super::model::{Comment, CreateComment};
use crate::app::profile::model::Profile;
use crate::app::profile::service::{fetch_profile_by_id, FetchProfileById};
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct CreateCommentService {
    pub body: String,
    pub article_id: Uuid,
    pub author: User,
}
pub fn create(
    conn: &PgConnection,
    params: &CreateCommentService,
) -> Result<(Comment, Profile), AppError> {
    let CreateCommentService {
        body,
        article_id,
        author,
    } = params;
    let comment = Comment::create(
        &conn,
        &CreateComment {
            body: body.to_string(),
            author_id: author.id,
            article_id: article_id.to_owned(),
        },
    )?;
    let profile = fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            id: author.id,
            me: author.to_owned(),
        },
    )?;
    Ok((comment, profile))
}

pub fn fetch_comments_list(
    conn: &PgConnection,
    me: &User,
) -> Result<Vec<(Comment, Profile)>, AppError> {
    use crate::schema::comments;
    use crate::schema::comments::dsl::*;
    use crate::schema::follows;
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
            let profile = fetch_profile_by_id(
                &conn,
                &FetchProfileById {
                    me: me.to_owned(),
                    id: _user.id,
                },
            )
            .expect("couldn't fetch profile."); // TODO: use ? or error handling
            (_comment.to_owned(), profile)
        })
        .collect::<Vec<(Comment, Profile)>>();

    Ok(_comments)
}
