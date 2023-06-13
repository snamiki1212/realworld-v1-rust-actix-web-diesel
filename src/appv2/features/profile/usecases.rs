use super::presenters::ProfilePresenter;
use super::repositories::ProfileRepository;
use crate::appv2::features::user::entities::User;
use crate::appv2::features::user::repositories::UserRepository;
use crate::error::AppError;
use actix_web::HttpResponse;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProfileUsecase {
    user_repository: Arc<dyn UserRepository>,
    profile_repository: ProfileRepository,
    presenter: ProfilePresenter,
}

impl ProfileUsecase {
    pub fn new(
        (profile_repository, user_repository): (ProfileRepository, Arc<dyn UserRepository>),
        presenter: ProfilePresenter,
    ) -> Self {
        Self {
            profile_repository,
            user_repository,
            presenter,
        }
    }

    pub fn show(&self, current_user: &User, username: &str) -> Result<HttpResponse, AppError> {
        let profile = self
            .profile_repository
            .fetch_by_name(current_user, username)?;
        Ok(self.presenter.from_profile(profile))
    }

    pub fn follow(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<HttpResponse, AppError> {
        let profile = self.user_repository.follow(current_user, target_username)?;
        Ok(self.presenter.from_profile(profile))
    }

    pub fn unfollow(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<HttpResponse, AppError> {
        let profile = self
            .user_repository
            .unfollow(current_user, target_username)?;
        Ok(self.presenter.from_profile(profile))
    }
}
