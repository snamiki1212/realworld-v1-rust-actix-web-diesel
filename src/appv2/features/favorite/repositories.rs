use super::entities::FavoriteInfo;
use super::services;
use crate::appv2::features::article::entities::Article;
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::Tag;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct FavoriteRepository {
    pool: DbPool,
}

impl FavoriteRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn favorite(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;
        let (article, profile, favorite_info, tags_list) = services::favorite(
            conn,
            &services::FavoriteService {
                current_user: user,
                article_title_slug,
            },
        )?;
        Ok((article, profile, favorite_info, tags_list))
    }

    pub fn unfavorite(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;
        let (article, profile, favorite_info, tags_list) = services::unfavorite(
            conn,
            &services::UnfavoriteService {
                current_user: user,
                article_title_slug,
            },
        )?;
        Ok((article, profile, favorite_info, tags_list))
    }
}
