use super::presenters::CommentPresenter;
use super::repositories::CommentRepository;
use crate::app::features::user::entities::User;
use crate::error::AppError;
use actix_web::HttpResponse;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CommentUsecase {
    comment_repository: Arc<dyn CommentRepository>,
    comment_presenter: Arc<dyn CommentPresenter>,
}

impl CommentUsecase {
    pub fn new(
        comment_repository: Arc<dyn CommentRepository>,
        comment_presenter: Arc<dyn CommentPresenter>,
    ) -> Self {
        Self {
            comment_repository,
            comment_presenter,
        }
    }

    pub fn fetch_comments(&self, user: &Option<User>) -> Result<HttpResponse, AppError> {
        let result = self.comment_repository.fetch_comments(user)?;
        let res = self.comment_presenter.to_multi_json(result);
        Ok(res)
    }

    pub fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .comment_repository
            .create_comment(body, article_title_slug, author)?;
        let res = self.comment_presenter.to_single_json(result);
        Ok(res)
    }

    pub fn delete_comment(
        &self,
        article_title_slug: &str,
        comment_id: Uuid,
        author_id: Uuid,
    ) -> Result<HttpResponse, AppError> {
        let _ = self
            .comment_repository
            .delete_comment(article_title_slug, comment_id, author_id);
        let res = self.comment_presenter.to_http_res();
        Ok(res)
    }
}
