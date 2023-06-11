use super::super::adapters::presenters::{ProfilePresenter, ProfileResponse};
use super::super::domains::profile_repository::ProfileRepository;
use crate::app::user::model::User;
use crate::appv2::features::user::domains::user_repository::UserRepository;
use crate::error::AppError;

pub struct ProfileUsecase {
    userRepository: UserRepository,
    profileRepository: ProfileRepository,
    presenter: ProfilePresenter,
}

impl ProfileUsecase {
    pub fn new(
        (profileRepository, userRepository): (ProfileRepository, UserRepository),
        presenter: ProfilePresenter,
    ) -> Self {
        Self {
            profileRepository,
            userRepository,
            presenter,
        }
    }

    pub fn show(&self, current_user: &User, username: &str) -> Result<ProfileResponse, AppError> {
        let profile = self
            .profileRepository
            .fetch_by_name(current_user, username)?;
        Ok(self.presenter.complete(profile))
    }

    pub fn follow(&self, current_user: &User, username: &str) -> Result<ProfileResponse, AppError> {
        let profile = current_user.follow(conn, &username)?;
        Ok(self.presenter.complete(profile))
    }
}
