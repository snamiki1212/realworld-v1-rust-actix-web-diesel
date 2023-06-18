use super::entities::Profile;
use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

pub trait ProfileRepository: Send + Sync + 'static {
    fn fetch_profile_by_name(
        &self,
        current_user: &User,
        username: &str,
    ) -> Result<Profile, AppError>;
}

#[derive(Clone)]
pub struct ProfileRepositoryImpl {
    pool: DbPool,
}

impl ProfileRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ProfileRepository for ProfileRepositoryImpl {
    fn fetch_profile_by_name(
        &self,
        current_user: &User,
        username: &str,
    ) -> Result<Profile, AppError> {
        let conn = &mut self.pool.get()?;
        let profile = {
            let followee = User::find_by_username(conn, username)?;
            current_user.fetch_profile(conn, &followee.id)?
        };
        Ok(profile)
    }
}
