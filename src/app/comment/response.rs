use crate::app::comment::model::Comment;
use crate::app::profile::model::Profile;
use chrono::NaiveDateTime;
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
                createdAt: comment.created_at,
                updatedAt: comment.updated_at,
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
                        createdAt: comment.created_at,
                        updatedAt: comment.updated_at,
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
pub struct InnerComment {
    pub id: Uuid,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
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
