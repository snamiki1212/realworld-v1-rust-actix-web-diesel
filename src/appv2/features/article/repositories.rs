use diesel::PgConnection;
use uuid::Uuid;

use super::entities::{Article, CreateArticle, DeleteArticle, UpdateArticle};
use super::services::{self, FetchArticlesListResult};
use crate::appv2::features::favorite::entities::FavoriteInfo;
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::{CreateTag, Tag};
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct ArticleRepository {
    pool: DbPool,
}

impl ArticleRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn fetch_articles_list(
        &self,
        params: services::FetchArticlesList,
    ) -> Result<FetchArticlesListResult, AppError> {
        let conn = &mut self.pool.get()?;
        let result = services::fetch_articles_list(
            conn,
            services::FetchArticlesList {
                tag: params.tag.clone(),
                author: params.author.clone(),
                favorited: params.favorited.clone(),
                offset: params.offset,
                limit: params.limit,
            },
        )?;
        Ok(result)
    }

    pub fn fetch_article_by_slug(
        &self,
        article_title_slug: String,
    ) -> Result<FetchArticleBySlugOutput, AppError> {
        let conn = &mut self.pool.get()?;

        let (article, author) = Article::fetch_by_slug_with_author(conn, &article_title_slug)?;

        let profile = author.fetch_profile(conn, &author.id)?;

        let tags_list = {
            use diesel::prelude::*;
            Tag::belonging_to(&article).load::<Tag>(conn)?
        };

        let favorite_info = {
            let is_favorited = article.is_favorited_by_user_id(conn, &author.id)?;
            let favorites_count = article.fetch_favorites_count(conn)?;
            FavoriteInfo {
                is_favorited,
                favorites_count,
            }
        };

        Ok((article, profile, favorite_info, tags_list))
    }

    pub fn create(
        &self,
        params: CreateArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;

        let article = Article::create(
            conn,
            &CreateArticle {
                author_id: params.current_user.id,
                slug: params.slug.clone(),
                title: params.title.clone(),
                description: params.description.clone(),
                body: params.body.clone(),
            },
        )?;

        let tag_list = Self::create_tag_list(conn, &params.tag_name_list, &article.id)?;

        let profile = params
            .current_user
            .fetch_profile(conn, &article.author_id)?;

        let favorite_info = {
            let is_favorited = article.is_favorited_by_user_id(conn, &params.current_user.id)?;
            let favorites_count = article.fetch_favorites_count(conn)?;
            FavoriteInfo {
                is_favorited,
                favorites_count,
            }
        };

        Ok((article, profile, favorite_info, tag_list))
    }

    pub fn delete(&self, input: DeleteArticleRepositoryInput) -> Result<(), AppError> {
        let conn = &mut self.pool.get()?;
        Article::delete(
            conn,
            &DeleteArticle {
                slug: input.slug,
                author_id: input.author_id,
            },
        )
    }

    pub fn update(
        &self,
        input: UpdateArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;

        let article = Article::update(
            conn,
            &input.article_title_slug,
            &input.current_user.id,
            &UpdateArticle {
                slug: input.slug.to_owned(),
                title: input.title.to_owned(),
                description: input.description.to_owned(),
                body: input.body.to_owned(),
            },
        )?;

        let tag_list = Tag::fetch_by_article_id(conn, &article.id)?;

        let profile = input.current_user.fetch_profile(conn, &article.author_id)?;

        let favorite_info = {
            let is_favorited = article.is_favorited_by_user_id(conn, &input.current_user.id)?;
            let favorites_count = article.fetch_favorites_count(conn)?;
            FavoriteInfo {
                is_favorited,
                favorites_count,
            }
        };

        Ok((article, profile, favorite_info, tag_list))
    }

    fn create_tag_list(
        conn: &mut PgConnection,
        tag_name_list: &Option<Vec<String>>,
        article_id: &Uuid,
    ) -> Result<Vec<Tag>, AppError> {
        let list = tag_name_list
            .as_ref()
            .map(|tag_name_list| {
                let records = tag_name_list
                    .iter()
                    .map(|name| CreateTag { name, article_id })
                    .collect();
                Tag::create_list(conn, records)
            })
            .unwrap_or_else(|| Ok(vec![]));
        list
    }

    pub fn fetch_article_item(
        &self,
        input: &FetchArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;
        let (article, author) = Article::find_with_author(conn, &input.article_id)?;

        let profile = input.current_user.fetch_profile(conn, &author.id)?;

        let favorite_info = {
            let is_favorited = article.is_favorited_by_user_id(conn, &input.current_user.id)?;
            let favorites_count = article.fetch_favorites_count(conn)?;
            FavoriteInfo {
                is_favorited,
                favorites_count,
            }
        };

        let tags_list = {
            use diesel::prelude::*;
            Tag::belonging_to(&article).load::<Tag>(conn)?
        };

        Ok((article, profile, favorite_info, tags_list))
    }
}

pub struct CreateArticleRepositoryInput {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
    pub current_user: User,
}

pub struct DeleteArticleRepositoryInput {
    pub slug: String,
    pub author_id: Uuid,
}

pub struct UpdateArticleRepositoryInput {
    pub current_user: User,
    pub article_title_slug: String,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

pub type FetchArticleBySlugOutput = (Article, Profile, FavoriteInfo, Vec<Tag>);

pub struct FetchArticleRepositoryInput {
    pub article_id: Uuid,
    pub current_user: User,
}
