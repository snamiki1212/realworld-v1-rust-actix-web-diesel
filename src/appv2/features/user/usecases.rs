use super::presenters::UserPresenter;
use super::repositories::UserRepository;
use crate::error::AppError;
use actix_web::HttpResponse;

#[derive(Clone)]
pub struct UserUsecase {
    user_repository: UserRepository,
    user_presenter: UserPresenter,
}

impl UserUsecase {
    pub fn new(user_repository: UserRepository, user_presenter: UserPresenter) -> Self {
        Self {
            user_repository,
            user_presenter,
        }
    }

    pub fn signin(&self, email: &str, password: &str) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.signin(email, password)?;
        let res = self.user_presenter.signin(user, token);
        Ok(res)
    }
}
