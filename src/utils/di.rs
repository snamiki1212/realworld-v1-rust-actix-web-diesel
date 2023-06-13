use crate::app::features::article::presenters::ArticlePresenterImpl;
use crate::app::features::article::repositories::ArticleRepositoryImpl;
use crate::app::features::article::usecases::ArticleUsecase;
use crate::app::features::comment::presenters::CommentPresenterImpl;
use crate::app::features::comment::repositories::CommentRepositoryImpl;
use crate::app::features::comment::usecases::CommentUsecase;
use crate::app::features::favorite::presenters::FavoritePresenterImpl;
use crate::app::features::favorite::repositories::FavoriteRepositoryImpl;
use crate::app::features::favorite::usecases::FavoriteUsecase;
use crate::app::features::profile::presenters::ProfilePresenterImpl;
use crate::app::features::profile::repositories::ProfileRepositoryImpl;
use crate::app::features::profile::usecases::ProfileUsecase;
use crate::app::features::tag::presenters::TagPresenterImpl;
use crate::app::features::tag::repositories::TagRepositoryImpl;
use crate::app::features::tag::usecases::TagUsecase;
use crate::app::features::user::presenters::UserPresenterImpl;
use crate::app::features::user::repositories::UserRepositoryImpl;
use crate::app::features::user::usecases::UserUsecase;
use std::sync::Arc;

use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UserRepositoryImpl,
    pub user_usecase: UserUsecase,
    pub user_presenter: UserPresenterImpl,

    /**
     * Profile
     */
    pub profile_repository: ProfileRepositoryImpl,
    pub profile_presenter: ProfilePresenterImpl,
    pub profile_usecase: ProfileUsecase,

    /**
     * Favorite
     */
    pub favorite_repository: FavoriteRepositoryImpl,
    pub favorite_presenter: FavoritePresenterImpl,
    pub favorite_usecase: FavoriteUsecase,

    /**
     * Article
     */
    pub article_repository: ArticleRepositoryImpl,
    pub article_presenter: ArticlePresenterImpl,
    pub article_usecase: ArticleUsecase,

    /**
     * Tag
     */
    pub tag_repository: TagRepositoryImpl,
    pub tag_presenter: TagPresenterImpl,
    pub tag_usecase: TagUsecase,

    /**
     * Comment
     */
    pub comment_repository: CommentRepositoryImpl,
    pub comment_presenter: CommentPresenterImpl,
    pub comment_usecase: CommentUsecase,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let user_repository = UserRepositoryImpl::new(pool.clone());
        let profile_repository = ProfileRepositoryImpl::new(pool.clone());
        let favorite_repository = FavoriteRepositoryImpl::new(pool.clone());
        let article_repository = ArticleRepositoryImpl::new(pool.clone());
        let tag_repository = TagRepositoryImpl::new(pool.clone());
        let comment_repository = CommentRepositoryImpl::new(pool.clone());

        // Presenter
        let user_presenter = UserPresenterImpl::new();
        let profile_presenter = ProfilePresenterImpl::new();
        let favorite_presenter = FavoritePresenterImpl::new();
        let article_presenter = ArticlePresenterImpl::new();
        let tag_presenter = TagPresenterImpl::new();
        let comment_presenter = CommentPresenterImpl::new();

        // Usecase
        let user_usecase = UserUsecase::new(
            Arc::new(user_repository.clone()),
            Arc::new(user_presenter.clone()),
        );
        let profile_usecase = ProfileUsecase::new(
            Arc::new(profile_repository.clone()),
            Arc::new(user_repository.clone()),
            Arc::new(profile_presenter.clone()),
        );
        let favorite_usecase = FavoriteUsecase::new(
            Arc::new(favorite_repository.clone()),
            Arc::new(favorite_presenter.clone()),
            Arc::new(article_repository.clone()),
        );
        let article_usecase = ArticleUsecase::new(
            Arc::new(article_repository.clone()),
            Arc::new(article_presenter.clone()),
        );
        let tag_usecase = TagUsecase::new(
            Arc::new(tag_repository.clone()),
            Arc::new(tag_presenter.clone()),
        );
        let comment_usecase = CommentUsecase::new(
            Arc::new(comment_repository.clone()),
            Arc::new(comment_presenter.clone()),
        );

        Self {
            // User
            user_repository,
            user_usecase,
            user_presenter,

            // Profile
            profile_presenter,
            profile_repository,
            profile_usecase,

            // Favorite
            favorite_repository,
            favorite_presenter,
            favorite_usecase,

            // Article
            article_repository,
            article_presenter,
            article_usecase,

            // Tag
            tag_repository,
            tag_presenter,
            tag_usecase,

            // Comment
            comment_repository,
            comment_presenter,
            comment_usecase,
        }
    }
}
