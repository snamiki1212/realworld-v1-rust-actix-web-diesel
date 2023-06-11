use crate::appv2::features::article::entities::{Article, FetchBySlugAndAuthorId};
use crate::appv2::features::article::services::{fetch_article, FetchArticle};
use crate::appv2::features::favorite::entities::{
    CreateFavorite, DeleteFavorite, Favorite, FavoriteInfo,
};
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::Tag;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use diesel::pg::PgConnection;

pub struct FavoriteService {
    pub current_user: User,
    pub article_title_slug: String,
}

#[deprecated(note = "use repository instead")]
pub fn favorite(
    conn: &mut PgConnection,
    params: &FavoriteService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let article = Article::fetch_by_slug_and_author_id(
        conn,
        &FetchBySlugAndAuthorId {
            slug: params.article_title_slug.to_owned(),
            author_id: params.current_user.id,
        },
    )?;
    Favorite::create(
        conn,
        &CreateFavorite {
            user_id: params.current_user.id,
            article_id: article.id,
        },
    )?;
    let item = fetch_article(
        conn,
        &FetchArticle {
            article_id: article.id,
            current_user: params.current_user.to_owned(),
        },
    )?;
    Ok(item)
}

pub struct UnfavoriteService {
    pub current_user: User,
    pub article_title_slug: String,
}

#[deprecated(note = "use repository instead")]
pub fn unfavorite(
    conn: &mut PgConnection,
    params: &UnfavoriteService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let article = Article::fetch_by_slug_and_author_id(
        conn,
        &FetchBySlugAndAuthorId {
            slug: params.article_title_slug.to_owned(),
            author_id: params.current_user.id,
        },
    )?;
    Favorite::delete(
        conn,
        &DeleteFavorite {
            user_id: params.current_user.id,
            article_id: article.id,
        },
    )?;
    let item = fetch_article(
        conn,
        &FetchArticle {
            article_id: article.id,
            current_user: params.current_user.to_owned(),
        },
    )?;
    Ok(item)
}
