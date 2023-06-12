use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct CommentRepository {
    pool: DbPool,
}

impl CommentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // pub fn favorite(&self, user: User, article_title_slug: String) -> Result<Article, AppError> {
    //     // let conn = &mut self.pool.get()?;

    //     // let article = Article::fetch_by_slug_and_author_id(
    //     //     conn,
    //     //     &FetchBySlugAndAuthorId {
    //     //         slug: article_title_slug.to_owned(),
    //     //         author_id: user.id,
    //     //     },
    //     // )?;
    //     // Favorite::create(
    //     //     conn,
    //     //     &CreateFavorite {
    //     //         user_id: user.id,
    //     //         article_id: article.id,
    //     //     },
    //     // )?;

    //     // Ok(article)
    // }
}
