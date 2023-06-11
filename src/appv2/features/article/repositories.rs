use super::services::{self, FetchArticlesListResult};
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
}
