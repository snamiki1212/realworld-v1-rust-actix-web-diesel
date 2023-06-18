use super::entities::{UpdateUser, User};
use super::presenters::UserPresenter;
use super::repositories::UserRepository;
use crate::error::AppError;
use actix_web::HttpResponse;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserUsecase {
    user_repository: Arc<dyn UserRepository>,
    user_presenter: Arc<dyn UserPresenter>,
}

impl UserUsecase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        user_presenter: Arc<dyn UserPresenter>,
    ) -> Self {
        Self {
            user_repository,
            user_presenter,
        }
    }

    pub fn signin(&self, email: &str, password: &str) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.signin(email, password)?;
        let res = self.user_presenter.to_json(user, token);
        Ok(res)
    }

    pub fn signup(
        &self,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.signup(email, username, password)?;
        let res = self.user_presenter.to_json(user, token);
        Ok(res)
    }

    pub fn get_token(&self, current_user: &User) -> Result<HttpResponse, AppError> {
        let token = current_user.generate_token()?;
        let res = self.user_presenter.to_json(current_user.clone(), token);
        Ok(res)
    }

    pub fn update_user(
        &self,
        user_id: Uuid,
        changeset: UpdateUser,
    ) -> Result<HttpResponse, AppError> {
        let (new_user, token) = self.user_repository.update(user_id, changeset)?;
        let res = self.user_presenter.to_json(new_user, token);
        Ok(res)
    }

    pub fn find_auth_user(&self, user_id: Uuid) -> Result<User, &str> {
        let maybe_user = self.user_repository.find(user_id);
        self.user_presenter.to_auth_middleware(maybe_user)
    }
}
