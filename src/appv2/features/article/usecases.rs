use super::presenters::ArticlePresenter;
use super::repositories::ArticleRepository;
use super::services;
use crate::error::AppError;
use actix_web::HttpResponse;

#[derive(Clone)]
pub struct ArticleUsecase {
    article_repository: ArticleRepository,
    article_presenter: ArticlePresenter,
}

impl ArticleUsecase {
    pub fn new(article_repository: ArticleRepository, article_presenter: ArticlePresenter) -> Self {
        Self {
            article_repository,
            article_presenter,
        }
    }

    pub fn fetch_articles_list(
        &self,
        params: services::FetchArticlesList,
    ) -> Result<HttpResponse, AppError> {
        let (list, count) =
            self.article_repository
                .fetch_articles_list(services::FetchArticlesList {
                    tag: params.tag.clone(),
                    author: params.author.clone(),
                    favorited: params.favorited.clone(),
                    offset: params.offset,
                    limit: params.limit,
                })?;
        let res = self.article_presenter.from_list_and_count(list, count);
        Ok(res)
    }

    pub fn fetch_article_by_slug(
        &self,
        params: &services::FetchArticleBySlug,
    ) -> Result<HttpResponse, AppError> {
        let article_title_slug = params.article_title_slug.clone();
        let result = self
            .article_repository
            .fetch_article_by_slug(article_title_slug)?;
        let res = self.article_presenter.from_item(result);
        Ok(res)
    }
}
