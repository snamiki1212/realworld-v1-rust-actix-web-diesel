use super::super::adapters::presenters::{ProfilePresenter, ProfileResponse};
use super::super::domains::profile_repository::ProfileRepository;
use crate::app::user::model::User;
use crate::appv2::features::user::domains::user_repository::UserRepository;
use crate::error::AppError;

pub struct ProfileUsecase {
    user_repository: UserRepository,
    profile_repository: ProfileRepository,
    presenter: ProfilePresenter,
}

impl ProfileUsecase {
    pub fn new(
        (profile_repository, user_repository): (ProfileRepository, UserRepository),
        presenter: ProfilePresenter,
    ) -> Self {
        Self {
            profile_repository,
            user_repository,
            presenter,
        }
    }

    pub fn show(&self, current_user: &User, username: &str) -> Result<ProfileResponse, AppError> {
        let profile = self
            .profile_repository
            .fetch_by_name(current_user, username)?;
        Ok(self.presenter.complete(profile))
    }

    pub fn follow(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<ProfileResponse, AppError> {
        let profile = self.user_repository.follow(current_user, target_username)?;
        Ok(self.presenter.complete(profile))
    }

    pub fn unfollow(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<ProfileResponse, AppError> {
        let profile = self
            .user_repository
            .unfollow(current_user, target_username)?;
        Ok(self.presenter.complete(profile))
    }
}
