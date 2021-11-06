use crate::app::article::model::Article;
use crate::app::article::service::{fetch_article, FetchArticle};
use crate::app::favorite::model::{Favorite, FavoriteInfo, FavorteAction, UnfavoriteAction};
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct FavoriteService {
    pub me: User,
    pub article_id: Uuid,
}
pub fn favorite(
    conn: &PgConnection,
    params: &FavoriteService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let _ = Favorite::favorite(
        conn,
        &FavorteAction {
            user_id: params.me.id,
            article_id: params.article_id,
        },
    )?;
    let item = fetch_article(
        conn,
        &FetchArticle {
            article_id: params.article_id,
            me: params.me.to_owned(),
        },
    )?;
    Ok(item)
}

pub struct UnfavoriteService {
    pub me: User,
    pub article_id: Uuid,
}
pub fn unfavorite(
    conn: &PgConnection,
    params: &UnfavoriteService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let item = fetch_article(
        conn,
        &FetchArticle {
            article_id: params.article_id,
            me: params.me.to_owned(),
        },
    )?;
    let _ = Favorite::unfavorite(
        conn,
        &UnfavoriteAction {
            user_id: params.me.id,
            article_id: params.article_id,
        },
    )?;
    Ok(item)
}

pub fn fetch_favorites_count_by_article_id(
    conn: &PgConnection,
    _article_id: Uuid,
) -> Result<i64, AppError> {
    use crate::schema::favorites;
    use diesel::prelude::*;
    let favorites_count = favorites::table
        .filter(favorites::article_id.eq_all(_article_id))
        .select(diesel::dsl::count(favorites::created_at))
        .first::<i64>(conn)?;
    Ok(favorites_count)
}

pub fn fetch_favorited_article_ids_by_user_id(
    conn: &PgConnection,
    user_id: Uuid,
) -> Result<Vec<Uuid>, AppError> {
    use crate::schema::favorites;
    use diesel::prelude::*;
    let favorited_article_ids = favorites::table
        .filter(favorites::user_id.eq(user_id))
        .select(favorites::user_id)
        .get_results::<Uuid>(conn)?;
    Ok(favorited_article_ids)
}
