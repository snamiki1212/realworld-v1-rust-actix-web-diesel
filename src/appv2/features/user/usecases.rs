use super::entities::User;
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
        let res = self.user_presenter.from_user_and_token(user, token);
        Ok(res)
    }

    pub fn signup(
        &self,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.signup(email, username, password)?;
        let res = self.user_presenter.from_user_and_token(user, token);
        Ok(res)
    }

    pub fn me(&self, current_user: &User) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.me(current_user)?;
        let res = self.user_presenter.from_user_and_token(user.clone(), token);
        Ok(res)
    }
}
