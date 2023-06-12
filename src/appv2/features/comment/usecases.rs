use crate::appv2::features::article::repositories::{
    ArticleRepository, FetchArticleRepositoryInput,
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

    // pub fn favorite(
    //     &self,
    //     user: User,
    //     article_title_slug: String,
    // ) -> Result<HttpResponse, AppError> {
    //     let article = self
    //         .favorite_repository
    //         .favorite(user.clone(), article_title_slug)?;

    //     let result = self
    //         .article_repository
    //         .fetch_article_item(&FetchArticleRepositoryInput {
    //             article_id: article.id,
    //             current_user: user,
    //         })?;
    //     let res = self.favorite_presenter.complete(result);
    //     Ok(res)
    // }
}
