// use crate::appv2::features::profile::domains::profile_repository::{
//     IProfileRepository, ProfileRepository,
// };
use crate::error::AppError;
use crate::utils::db::DbPool;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
// use std::sync::Arc;

type AppConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    // pub profile_repository: Arc<dyn IProfileRepository>,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        // let profile_repository = ProfileRepository::new(pool);
        Self {
            pool,
            // profile_repository: Arc::new(profile_repository),
        }
    }

    pub fn get_conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}
