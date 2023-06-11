// use crate::appv2::features::profile::domains::profile_repository::{
//     IProfileRepository, ProfileRepository,
// };
use crate::error::AppError;
use crate::utils::db::DbPool;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
// use std::sync::Arc;
use crate::utils::di::DiContainer;

type AppConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    #[deprecated]
    pub pool: DbPool,
    pub di_container: DiContainer,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        let di_container = DiContainer::new(&pool);
        Self { pool, di_container }
    }

    pub fn get_conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}
