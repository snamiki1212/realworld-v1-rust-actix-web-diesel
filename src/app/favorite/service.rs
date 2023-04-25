use crate::app::article::model::{Article, FetchBySlugAndAuthorId};
use crate::app::article::service::{fetch_article, FetchArticle};
use crate::app::favorite::model::{CreateFavorite, DeleteFavorite, Favorite, FavoriteInfo};
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;

pub struct FavoriteService {
    pub current_user: User,
    pub article_title_slug: String,
}

// TODO: move to User model
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
    let _ = Favorite::create(
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
    let _ = Favorite::delete(
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
