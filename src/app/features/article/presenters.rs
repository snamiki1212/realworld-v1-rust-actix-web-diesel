use super::{entities::Article, repositories::ArticlesList};
use crate::app::features::favorite::entities::FavoriteInfo;
use crate::app::features::profile::entities::Profile;
use crate::app::features::tag::entities::Tag;
use crate::utils::date::Iso8601;
use actix_web::HttpResponse;
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
                tag_list: tag_list
                    .iter()
                    .map(move |tag| tag.name.to_owned())
                    .collect(),
                created_at: Iso8601(article.created_at),
                updated_at: Iso8601(article.updated_at),
                favorited: favorite_info.is_favorited.to_owned(),
                favorites_count: favorite_info.favorites_count.to_owned(),
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
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articles_count: ArticleCount,
}

type ArticlesCount = i64;
type Inner = ((Article, Profile, FavoriteInfo), Vec<Tag>);
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
            articles_count,
            articles,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleContent {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: Iso8601,
    pub updated_at: Iso8601,
    pub favorited: bool,
    pub favorites_count: i64,
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
            tag_list: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            created_at: Iso8601(article.created_at),
            updated_at: Iso8601(article.updated_at),
            favorited: favorite_info.is_favorited.to_owned(),
            favorites_count: favorite_info.favorites_count.to_owned(),
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

pub trait ArticlePresenter: Send + Sync + 'static {
    fn to_multi_json(&self, list: ArticlesList, count: i64) -> HttpResponse;
    fn to_single_json(&self, item: (Article, Profile, FavoriteInfo, Vec<Tag>)) -> HttpResponse;
    fn to_http_res(&self) -> HttpResponse;
}

#[derive(Clone)]
pub struct ArticlePresenterImpl {}
impl ArticlePresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl ArticlePresenter for ArticlePresenterImpl {
    fn to_multi_json(&self, list: ArticlesList, count: i64) -> HttpResponse {
        let res = MultipleArticlesResponse::from((list, count));
        HttpResponse::Ok().json(res)
    }
    fn to_single_json(&self, item: (Article, Profile, FavoriteInfo, Vec<Tag>)) -> HttpResponse {
        let res = SingleArticleResponse::from(item);
        HttpResponse::Ok().json(res)
    }
    fn to_http_res(&self) -> HttpResponse {
        HttpResponse::Ok().json(())
    }
}
