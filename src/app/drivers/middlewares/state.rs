use crate::utils::db::DbPool;
use crate::utils::di::DiContainer;

#[derive(Clone)]
pub struct AppState {
    pub di_container: DiContainer,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        let di_container = DiContainer::new(&pool);
        Self { di_container }
    }
}
