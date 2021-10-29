use super::model::{Comment, CreateComment};
use crate::app::profile::model::Profile;
use crate::app::profile::service::{fetch_profile_by_id, FetchProfileById};
use crate::app::user::model::User;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct CreateCommentService {
    pub body: String,
    pub article_id: Uuid,
    pub author: User,
}
pub fn create(conn: &PgConnection, params: &CreateCommentService) -> (Comment, Profile) {
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
    );
    let profile = fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            id: author.id,
            me: author.to_owned(),
        },
    );
    (comment, profile)
}
