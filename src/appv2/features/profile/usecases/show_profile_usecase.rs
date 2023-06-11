use super::super::adapters::presenters::{ProfilePresenter, ProfileResponse};
use super::super::domains::profile_repository::ProfileRepository;
use crate::app::user::model::User;
use crate::error::AppError;

pub struct ShowProfileUsecase {
    repo: ProfileRepository,
    presenter: ProfilePresenter,
}

impl ShowProfileUsecase {
    pub fn new(repo: ProfileRepository, presenter: ProfilePresenter) -> Self {
        Self { repo, presenter }
    }

    pub fn handle(&self, current_user: &User, username: &str) -> Result<ProfileResponse, AppError> {
        let profile = self.repo.fetch_by_name(current_user, username)?;
        Ok(self.presenter.complete(profile))
    }
}
