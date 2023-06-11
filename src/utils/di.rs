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

        Self {
            user_repository,
            user_usecase,
            user_presenter,
            profile_presenter,
            profile_repository,
            profile_usecase,
        }
    }
}
