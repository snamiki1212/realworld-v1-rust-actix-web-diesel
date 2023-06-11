use crate::appv2::features::favorite::{
    presenters::FavoritePresenter, repositories::FavoriteRepository, usecases::FavoriteUsecase,
};
use crate::appv2::features::profile::{
    presenters::ProfilePresenter, repositories::ProfileRepository, usecases::ProfileUsecase,
};
use crate::appv2::features::user::{
    presenters::UserPresenter, repositories::UserRepository, usecases::UserUsecase,
};

use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UserRepository,
    pub user_usecase: UserUsecase,
    pub user_presenter: UserPresenter,

    /**
     * Profile
     */
    pub profile_repository: ProfileRepository,
    pub profile_presenter: ProfilePresenter,
    pub profile_usecase: ProfileUsecase,

    /**
     * Favorite
     */
    pub favorite_repository: FavoriteRepository,
    pub favorite_presenter: FavoritePresenter,
    pub favorite_usecase: FavoriteUsecase,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // User
        let user_repository = UserRepository::new(pool.clone());
        let user_presenter = UserPresenter::new();
        let user_usecase = UserUsecase::new(user_repository.clone(), user_presenter.clone());

        // Profile
        let profile_repository = ProfileRepository::new(pool.clone());
        let profile_presenter = ProfilePresenter::new();
        let profile_usecase = ProfileUsecase::new(
            (profile_repository.clone(), user_repository.clone()),
            profile_presenter.clone(),
        );

        // Favorite
        let favorite_repository = FavoriteRepository::new(pool.clone());
        let favorite_presenter = FavoritePresenter::new();
        let favorite_usecase =
            FavoriteUsecase::new(favorite_repository.clone(), favorite_presenter.clone());

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
        }
    }
}
