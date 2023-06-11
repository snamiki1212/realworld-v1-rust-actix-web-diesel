use super::entities::Tag;
use crate::error::AppError;
use crate::utils::db::DbPool;

type Token = String;

#[derive(Clone)]
pub struct TagRepository {
    pool: DbPool,
}

impl TagRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn list(&self) -> Result<Vec<Tag>, AppError> {
        let conn = &mut self.pool.get()?;
        Tag::fetch(conn)
    }
}
