use crate::app::article::model::Article;
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};
use std::convert::From;
type ArticleCount = i64;

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl From<(Article, Profile, Vec<Tag>)> for SingleArticleResponse {
    fn from((article, profile, tag_list): (Article, Profile, Vec<Tag>)) -> Self {
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
                favorited: false,  // TODO: fix
                favoritesCount: 0, // TODO: fix
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

type IsFavorited = bool;
type FavoritedCount = i64;
type ArticlesCount = i64;
type Inner = (((Article, Profile, IsFavorited), FavoritedCount), Vec<Tag>);
type ArticlesList = Vec<Inner>;
type Item = (ArticlesList, ArticlesCount);
impl From<Item> for MultipleArticlesResponse {
    fn from((list, articles_count): (Vec<Inner>, ArticleCount)) -> Self {
        let articles = list
            .iter()
            .map(
                |(((article, profile, isFavorited), favorited_count), tags_list)| {
                    ArticleContent::from((
                        article.to_owned(),
                        profile.to_owned(),
                        isFavorited.to_owned(),
                        favorited_count.to_owned(),
                        tags_list.to_owned(),
                    ))
                },
            )
            .collect();
        Self {
            articlesCount: articles_count,
            articles: articles,
        }
    }
}

type DEPRECATED_Info = ((Article, Profile), Vec<Tag>);
impl MultipleArticlesResponse {
    pub fn DEPRECATED_from((info, articles_count): (Vec<DEPRECATED_Info>, ArticleCount)) -> Self {
        let articles = info
            .iter()
            .map(|((article, profile), tags_list)| {
                ArticleContent::DEPRECATED_from(
                    article.to_owned(),
                    profile.to_owned(),
                    tags_list.to_owned(),
                )
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

impl ArticleContent {
    pub fn DEPRECATED_from(article: Article, profile: Profile, tag_list: Vec<Tag>) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tagList: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            createdAt: article.created_at.to_string(),
            updatedAt: article.updated_at.to_string(),
            favorited: false,  // TODO: fix
            favoritesCount: 0, // TODO: fix
            author: AuthorContent {
                username: profile.username,
                bio: profile.bio,
                image: profile.image,
                following: profile.following,
            },
        }
    }
}

impl From<(Article, Profile, IsFavorited, FavoritedCount, Vec<Tag>)> for ArticleContent {
    fn from(
        (article, profile, is_favorited, favorited_count, tag_list): (
            Article,
            Profile,
            IsFavorited,
            FavoritedCount,
            Vec<Tag>,
        ),
    ) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tagList: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            createdAt: article.created_at.to_string(),
            updatedAt: article.updated_at.to_string(),
            favorited: is_favorited,
            favoritesCount: favorited_count,
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
