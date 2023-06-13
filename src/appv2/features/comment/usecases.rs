use crate::appv2::features::article::repositories::{
    ArticleRepositoryImpl, FetchArticleRepositoryInput,
};
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use actix_web::HttpResponse;
use uuid::Uuid;

use super::presenters::CommentPresenter;
use super::repositories::CommentRepository;

#[derive(Clone)]
pub struct CommentUsecase {
    comment_repository: CommentRepository,
    comment_presenter: CommentPresenter,
}

impl CommentUsecase {
    pub fn new(comment_repository: CommentRepository, comment_presenter: CommentPresenter) -> Self {
        Self {
            comment_repository,
            comment_presenter,
        }
    }

    pub fn fetch_comments_list(&self, user: &Option<User>) -> Result<HttpResponse, AppError> {
        let result = self.comment_repository.fetch_comments_list(user)?;
        let res = self.comment_presenter.from_comment_and_profile_list(result);
        Ok(res)
    }

    pub fn create(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .comment_repository
            .create(body, article_title_slug, author)?;
        let res = self.comment_presenter.from_comment_and_profile(result);
        Ok(res)
    }

    pub fn delete(
        &self,
        article_title_slug: &str,
        comment_id: Uuid,
        author_id: Uuid,
    ) -> Result<HttpResponse, AppError> {
        self.comment_repository
            .delete(&article_title_slug, comment_id, author_id);
        let res = self.comment_presenter.toHttpRes();
        Ok(res)
    }
}
