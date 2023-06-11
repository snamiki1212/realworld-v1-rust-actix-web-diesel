use super::entities::Profile;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

pub trait IProfileRepository {
    fn fetch_by_name(&self, current_user: &User, username: &str) -> Result<Profile, AppError>;
}

#[derive(Clone)]
pub struct ProfileRepository {
    pool: DbPool,
}

impl ProfileRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn fetch_by_name(&self, current_user: &User, username: &str) -> Result<Profile, AppError> {
        let conn = &mut self.pool.get()?;
        let profile = {
            let followee = User::find_by_username(conn, username)?;
            current_user.fetch_profile(conn, &followee.id)?
        };
        Ok(profile)
    }
}
