use crate::appv2::features::comment::entities::Comment;
use crate::appv2::features::profile::entities::Profile;
use crate::utils::date::Iso8601;
use actix_web::HttpResponse;
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

#[derive(Clone)]
pub struct CommentPresenter {}
impl CommentPresenter {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn complete(
    //     &self,
    //     (article, profile, favorite_info, tags_list): (Article, Profile, FavoriteInfo, Vec<Tag>),
    // ) -> HttpResponse {
    //     let res_model = SingleArticleResponse::from((article, profile, favorite_info, tags_list));
    //     HttpResponse::Ok().json(res_model)
    // }
    pub fn toHttpRes(&self) -> HttpResponse {
        HttpResponse::Ok().json("OK")
    }

    pub fn from_comment_and_profile_list(&self, list: Vec<(Comment, Profile)>) -> HttpResponse {
        let res = MultipleCommentsResponse::from(list);
        HttpResponse::Ok().json(res)
    }

    pub fn from_comment_and_profile(&self, item: (Comment, Profile)) -> HttpResponse {
        let res = SingleCommentResponse::from(item);
        HttpResponse::Ok().json(res)
    }
}