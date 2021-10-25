use super::model::Article;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl SingleArticleResponse {
    pub fn from(article: Article, user: User, tag_list: Vec<String>) -> Self {
        Self {
            article: ArticleContent {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tag_list: tag_list,
                createdAt: article.created_at.to_string(),
                updatedAt: article.updated_at.to_string(),
                author: AuthorContent {
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                    following: true, // TODO: get following by db
                },
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ArticleContent {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub createdAt: String,
    pub updatedAt: String,
    // pub favorited,
    // pub favoritesCount,
    pub author: AuthorContent,
}

#[derive(Deserialize, Serialize)]
pub struct AuthorContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
