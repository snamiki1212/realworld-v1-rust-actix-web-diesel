// use super::entities::{UpdateUser, User};
use super::presenters::FavoritePresenter;
use super::repositories::FavoriteRepository;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use actix_web::HttpResponse;
use uuid::Uuid;

#[derive(Clone)]
pub struct FavoriteUsecase {
    favorite_repository: FavoriteRepository,
    favorite_presenter: FavoritePresenter,
}

impl FavoriteUsecase {
    pub fn new(
        favorite_repository: FavoriteRepository,
        favorite_presenter: FavoritePresenter,
    ) -> Self {
        Self {
            favorite_repository,
            favorite_presenter,
        }
    }

    pub fn favorite(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .favorite_repository
            .favorite(user, article_title_slug)?;
        let res = self.favorite_presenter.complete(result);
        Ok(res)
    }

    pub fn unfavorite(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .favorite_repository
            .unfavorite(user, article_title_slug)?;
        let res = self.favorite_presenter.complete(result);
        Ok(res)
    }
}
