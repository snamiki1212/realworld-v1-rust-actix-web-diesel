use super::entities::Article;
use super::presenters::ArticlePresenter;
use super::repositories::{
    ArticleRepository, CreateArticleRepositoryInput, DeleteArticleRepositoryInput,
    FetchArticlesRepositoryInput, FetchFollowingArticlesRepositoryInput,
    UpdateArticleRepositoryInput,
};
use crate::app::features::user::entities::User;
use crate::error::AppError;
use actix_web::HttpResponse;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ArticleUsecase {
    article_repository: Arc<dyn ArticleRepository>,
    article_presenter: Arc<dyn ArticlePresenter>,
}

impl ArticleUsecase {
    pub fn new(
        article_repository: Arc<dyn ArticleRepository>,
        article_presenter: Arc<dyn ArticlePresenter>,
    ) -> Self {
        Self {
            article_repository,
            article_presenter,
        }
    }

    pub fn fetch_articles(
        &self,
        params: FetchArticlesUsecaseInput,
    ) -> Result<HttpResponse, AppError> {
        let (list, count) =
            self.article_repository
                .fetch_articles(FetchArticlesRepositoryInput {
                    tag: params.tag.clone(),
                    author: params.author.clone(),
                    favorited: params.favorited.clone(),
                    offset: params.offset,
                    limit: params.limit,
                })?;
        let res = self.article_presenter.to_multi_json(list, count);
        Ok(res)
    }

    pub fn fetch_article_by_slug(
        &self,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .article_repository
            .fetch_article_by_slug(article_title_slug)?;
        let res = self.article_presenter.to_single_json(result);
        Ok(res)
    }

    pub fn fetch_following_articles(
        &self,
        user: User,
        offset: i64,
        limit: i64,
    ) -> Result<HttpResponse, AppError> {
        let (list, count) = self.article_repository.fetch_following_articles(
            &FetchFollowingArticlesRepositoryInput {
                current_user: user,
                offset,
                limit,
            },
        )?;
        let res = self.article_presenter.to_multi_json(list, count);
        Ok(res)
    }

    pub fn create_article(
        &self,
        params: CreateArticleUsecaseInput,
    ) -> Result<HttpResponse, AppError> {
        let slug = Article::convert_title_to_slug(&params.title);
        let result = self
            .article_repository
            .create_article(CreateArticleRepositoryInput {
                body: params.body,
                current_user: params.current_user,
                description: params.description,
                tag_name_list: params.tag_name_list,
                title: params.title,
                slug,
            })?;
        let res = self.article_presenter.to_single_json(result);
        Ok(res)
    }

    pub fn delete_article(
        &self,
        input: DeleteArticleUsecaseInput,
    ) -> Result<HttpResponse, AppError> {
        self.article_repository
            .delete_article(DeleteArticleRepositoryInput {
                slug: input.slug,
                author_id: input.author_id,
            })?;
        let res = self.article_presenter.to_http_res();
        Ok(res)
    }

    pub fn update_article(
        &self,
        input: UpdateArticleUsecaseInput,
    ) -> Result<HttpResponse, AppError> {
        let article_slug = &input
            .title
            .as_ref()
            .map(|_title| Article::convert_title_to_slug(_title));
        let slug = article_slug.to_owned();
        let result = self
            .article_repository
            .update_article(UpdateArticleRepositoryInput {
                current_user: input.current_user,
                article_title_slug: input.article_title_slug,
                slug,
                title: input.title,
                description: input.description,
                body: input.body,
            })?;
        let res = self.article_presenter.to_single_json(result);
        Ok(res)
    }
}

pub struct CreateArticleUsecaseInput {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
    pub current_user: User,
}

pub struct DeleteArticleUsecaseInput {
    pub slug: String,
    pub author_id: Uuid,
}

pub struct UpdateArticleUsecaseInput {
    pub current_user: User,
    pub article_title_slug: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

pub struct FetchArticlesUsecaseInput {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}
