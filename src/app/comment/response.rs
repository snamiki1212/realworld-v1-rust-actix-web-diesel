use crate::app::comment::model::Comment;
use crate::app::profile::model::Profile;
use crate::utils::date::Iso8601;
use serde::{Deserialize, Serialize};
use std::convert::From;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SingleCommentResponse {
    pub comment: InnerComment,
}

impl From<(Comment, Profile)> for SingleCommentResponse {
    fn from((comment, profile): (Comment, Profile)) -> Self {
        Self {
            comment: InnerComment {
                id: comment.id,
                body: comment.body,
                author: InnerAuthor {
                    username: profile.username,
                    bio: profile.bio,
                    image: profile.image,
                    following: profile.following,
                },
                created_at: Iso8601(comment.created_at),
                updated_at: Iso8601(comment.updated_at),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MultipleCommentsResponse {
    pub comments: Vec<InnerComment>,
}

impl From<Vec<(Comment, Profile)>> for MultipleCommentsResponse {
    fn from(list: Vec<(Comment, Profile)>) -> Self {
        Self {
            comments: list
                .into_iter()
                .map(|item| {
                    let (comment, profile) = item;
                    InnerComment {
                        id: comment.id,
                        created_at: Iso8601(comment.created_at),
                        updated_at: Iso8601(comment.updated_at),
                        body: comment.body,
                        author: InnerAuthor {
                            username: profile.username,
                            bio: profile.bio,
                            image: profile.image,
                            following: profile.following,
                        },
                    }
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerComment {
    pub id: Uuid,
    pub created_at: Iso8601,
    pub updated_at: Iso8601,
    pub body: String,
    pub author: InnerAuthor,
}

#[derive(Serialize, Deserialize)]
pub struct InnerAuthor {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
