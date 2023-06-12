use super::entities::{Article, CreateArticle};
use super::services::{self, FetchArticlesListResult};
use crate::appv2::features::favorite::entities::FavoriteInfo;
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::Tag;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

type ArticleCount = i64;
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
    ) -> Result<services::FetchArticleBySlugResult, AppError> {
        let conn = &mut self.pool.get()?;
        services::fetch_article_by_slug(conn, &services::FetchArticleBySlug { article_title_slug })
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
        let tag_list = services::create_tag_list(conn, &params.tag_name_list, &article.id)?;

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
}

pub struct CreateArticleRepositoryInput {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
    pub current_user: User,
}
