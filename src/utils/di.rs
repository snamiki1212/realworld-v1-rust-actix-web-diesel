use crate::appv2::features::article::presenters::ArticlePresenter;
use crate::appv2::features::article::repositories::ArticleRepositoryImpl;
use crate::appv2::features::article::usecases::ArticleUsecase;
use crate::appv2::features::comment::presenters::CommentPresenter;
use crate::appv2::features::comment::repositories::CommentRepository;
use crate::appv2::features::comment::usecases::CommentUsecase;
use crate::appv2::features::favorite::presenters::FavoritePresenter;
use crate::appv2::features::favorite::repositories::FavoriteRepositoryImpl;
use crate::appv2::features::favorite::usecases::FavoriteUsecase;
use crate::appv2::features::profile::presenters::ProfilePresenter;
use crate::appv2::features::profile::repositories::ProfileRepositoryImpl;
use crate::appv2::features::profile::usecases::ProfileUsecase;
use crate::appv2::features::tag::presenters::TagPresenter;
use crate::appv2::features::tag::repositories::TagRepository;
use crate::appv2::features::tag::usecases::TagUsecase;
use crate::appv2::features::user::presenters::UserPresenter;
use crate::appv2::features::user::repositories::UserRepositoryImpl;
use crate::appv2::features::user::usecases::UserUsecase;
use std::sync::Arc;

use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UserRepositoryImpl,
    pub user_usecase: UserUsecase,
    pub user_presenter: UserPresenter,

    /**
     * Profile
     */
    pub profile_repository: ProfileRepositoryImpl,
    pub profile_presenter: ProfilePresenter,
    pub profile_usecase: ProfileUsecase,

    /**
     * Favorite
     */
    pub favorite_repository: FavoriteRepositoryImpl,
    pub favorite_presenter: FavoritePresenter,
    pub favorite_usecase: FavoriteUsecase,

    /**
     * Article
     */
    pub article_repository: ArticleRepositoryImpl,
    pub article_presenter: ArticlePresenter,
    pub article_usecase: ArticleUsecase,

    /**
     * Tag
     */
    pub tag_repository: TagRepository,
    pub tag_presenter: TagPresenter,
    pub tag_usecase: TagUsecase,

    /**
     * Comment
     */
    pub comment_repository: CommentRepository,
    pub comment_presenter: CommentPresenter,
    pub comment_usecase: CommentUsecase,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let user_repository = UserRepositoryImpl::new(pool.clone());
        let profile_repository = ProfileRepositoryImpl::new(pool.clone());
        let favorite_repository = FavoriteRepositoryImpl::new(pool.clone());
        let article_repository = ArticleRepositoryImpl::new(pool.clone());
        let tag_repository = TagRepository::new(pool.clone());
        let comment_repository = CommentRepository::new(pool.clone());

        // Presenter
        let user_presenter = UserPresenter::new();
        let profile_presenter = ProfilePresenter::new();
        let favorite_presenter = FavoritePresenter::new();
        let article_presenter = ArticlePresenter::new();
        let tag_presenter = TagPresenter::new();
        let comment_presenter = CommentPresenter::new();

        // Usecase
        let user_usecase =
            UserUsecase::new(Arc::new(user_repository.clone()), user_presenter.clone());
        let profile_usecase = ProfileUsecase::new(
            Arc::new(profile_repository.clone()),
            Arc::new(user_repository.clone()),
            profile_presenter.clone(),
        );
        let favorite_usecase = FavoriteUsecase::new(
            Arc::new(favorite_repository.clone()),
            favorite_presenter.clone(),
            Arc::new(article_repository.clone()),
        );
        let article_usecase = ArticleUsecase::new(
            Arc::new(article_repository.clone()),
            article_presenter.clone(),
        );
        let tag_usecase = TagUsecase::new(tag_repository.clone(), tag_presenter.clone());
        let comment_usecase =
            CommentUsecase::new(comment_repository.clone(), comment_presenter.clone());

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
