use crate::error::AppError;
use crate::utils::db::DbPool;
use crate::utils::di::DiContainer;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

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
