use crate::app::comment::model::Comment;
use crate::app::profile::model::Profile;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SingleCommentResponse {
    pub comment: InnerComment,
}

impl SingleCommentResponse {
    pub fn from(comment: Comment, profile: Profile) -> Self {
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
