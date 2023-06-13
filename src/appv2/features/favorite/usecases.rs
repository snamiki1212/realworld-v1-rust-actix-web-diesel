use super::presenters::FavoritePresenter;
use super::repositories::FavoriteRepository;
use crate::appv2::features::article::repositories::{
    ArticleRepository, FetchArticleRepositoryInput,
};
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use actix_web::HttpResponse;
use std::sync::Arc;

#[derive(Clone)]
pub struct FavoriteUsecase {
    favorite_repository: Arc<dyn FavoriteRepository>,
    favorite_presenter: FavoritePresenter,
    article_repository: ArticleRepository,
}

impl FavoriteUsecase {
    pub fn new(
        favorite_repository: Arc<dyn FavoriteRepository>,
        favorite_presenter: FavoritePresenter,
        article_repository: ArticleRepository,
    ) -> Self {
        Self {
            favorite_repository,
            favorite_presenter,
            article_repository,
        }
    }

    pub fn favorite(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let article = self
            .favorite_repository
            .favorite(user.clone(), article_title_slug)?;

        let result = self
            .article_repository
            .fetch_article_item(&FetchArticleRepositoryInput {
                article_id: article.id,
                current_user: user,
            })?;
        let res = self.favorite_presenter.complete(result);
        Ok(res)
    }

    pub fn unfavorite(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let article = self
            .favorite_repository
            .unfavorite(user.clone(), article_title_slug)?;

        let result = self
            .article_repository
            .fetch_article_item(&FetchArticleRepositoryInput {
                article_id: article.id,
                current_user: user,
            })?;

        let res = self.favorite_presenter.complete(result);
        Ok(res)
    }
}
