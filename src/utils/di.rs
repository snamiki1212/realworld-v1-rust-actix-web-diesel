use crate::appv2::features::profile::adapters::presenters::ProfilePresenter;
use crate::appv2::features::profile::domains::profile_repository::ProfileRepository;
use crate::appv2::features::profile::usecases::profile_usecase::ProfileUsecase;
use crate::appv2::features::user::domains::user_repository::UserRepository;

use crate::utils::db::DbPool;

pub struct DiContainer {
    /**
     * Profile
     */
    pub profile_repository: ProfileRepository,
    pub profile_presenter: ProfilePresenter,
    pub profile_usecase: ProfileUsecase,

    /**
     * User
     */
    pub user_repository: UserRepository,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        let profile_repository = ProfileRepository::new(pool.clone());
        let user_repository = UserRepository::new(pool.clone());
        let profile_presenter = ProfilePresenter::new();
        let profile_usecase = ProfileUsecase::new(
            (profile_repository.clone(), user_repository.clone()),
            profile_presenter.clone(),
        );
        Self {
            profile_presenter,
            profile_repository,
            profile_usecase,
            user_repository,
        }
    }
}
