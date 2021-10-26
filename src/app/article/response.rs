use crate::app::article::model::Article;
use crate::app::article::tag::model::Tag;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};

type ArticleCount = i64;

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl SingleArticleResponse {
    pub fn from(article: Article, user: User, tag_list: Vec<Tag>) -> Self {
        Self {
            article: ArticleContent::from(article, user, tag_list),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articlesCount: ArticleCount,
}

type Info = (Article, User, Tag);
impl MultipleArticlesResponse {
    pub fn from(info: Vec<Info>, user: User, articles_count: ArticleCount) -> Self {
        let articles = info
            .iter()
            .map(|(article, user, tags_list)| {
                ArticleContent::from(article.to_owned(), user.clone(), vec![tags_list.to_owned()])
            })
            .collect();
        Self {
            articlesCount: articles_count,
            articles: articles,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ArticleContent {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>, // TODO: tagList
    pub createdAt: String,
    pub updatedAt: String,
    // pub favorited,
    // pub favoritesCount,
    pub author: AuthorContent,
}

impl ArticleContent {
    pub fn from(article: Article, user: User, tag_list: Vec<Tag>) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            createdAt: article.created_at.to_string(),
            updatedAt: article.updated_at.to_string(),
            author: AuthorContent {
                username: user.username,
                bio: user.bio,
                image: user.image,
                following: true, // TODO: get following by db
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthorContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
