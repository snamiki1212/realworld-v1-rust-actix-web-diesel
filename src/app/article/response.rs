use crate::app::article::model::Article;
use crate::app::favorite::model::FavoriteInfo;
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use serde::{Deserialize, Serialize};
use std::convert::From;

type ArticleCount = i64;

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl From<(Article, Profile, FavoriteInfo, Vec<Tag>)> for SingleArticleResponse {
    fn from(
        (article, profile, favorite_info, tag_list): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> Self {
        Self {
            article: ArticleContent {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tagList: tag_list
                    .iter()
                    .map(move |tag| tag.name.to_owned())
                    .collect(),
                createdAt: article.created_at.to_string(),
                updatedAt: article.updated_at.to_string(),
                favorited: favorite_info.is_favorited.to_owned(),
                favoritesCount: favorite_info.favorites_count.to_owned(),
                author: AuthorContent {
                    username: profile.username,
                    bio: profile.bio,
                    image: profile.image,
                    following: profile.following,
                },
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articlesCount: ArticleCount,
}

type ArticlesCount = i64;
type Inner = ((Article, Profile, FavoriteInfo), Vec<Tag>);
type ArticlesList = Vec<Inner>;
type Item = (ArticlesList, ArticlesCount);
impl From<Item> for MultipleArticlesResponse {
    fn from((list, articles_count): (Vec<Inner>, ArticleCount)) -> Self {
        let articles = list
            .iter()
            .map(|((article, profile, favorite_info), tags_list)| {
                ArticleContent::from((
                    article.to_owned(),
                    profile.to_owned(),
                    favorite_info.to_owned(),
                    tags_list.to_owned(),
                ))
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
    pub tagList: Vec<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub favorited: bool,
    pub favoritesCount: i64,
    pub author: AuthorContent,
}

impl From<(Article, Profile, FavoriteInfo, Vec<Tag>)> for ArticleContent {
    fn from(
        (article, profile, favorite_info, tag_list): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tagList: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            createdAt: article.created_at.to_string(),
            updatedAt: article.updated_at.to_string(),
            favorited: favorite_info.is_favorited.to_owned(),
            favoritesCount: favorite_info.favorites_count.to_owned(),
            author: AuthorContent {
                username: profile.username,
                bio: profile.bio,
                image: profile.image,
                following: profile.following,
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
