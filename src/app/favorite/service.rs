use crate::app::article::model::Article;
use crate::app::article::service::{fetch_article, FetchArticle};
use crate::app::favorite::model::{Favorite, FavorteAction, UnfavoriteAction};
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct FavoriteService {
    pub me: User,
    pub article_id: Uuid,
}
pub fn favorite(conn: &PgConnection, params: &FavoriteService) -> (Article, Profile, Vec<Tag>) {
    let _ = Favorite::favorite(
        conn,
        &FavorteAction {
            user_id: params.me.id,
            article_id: params.article_id,
        },
    );
    let item = fetch_article(
        conn,
        &FetchArticle {
            article_id: params.article_id,
            me: params.me.to_owned(),
        },
    );
    item
}

pub struct UnfavoriteService {
    pub me: User,
    pub article_id: Uuid,
}
pub fn unfavorite(conn: &PgConnection, params: &UnfavoriteService) -> (Article, Profile, Vec<Tag>) {
    let item = fetch_article(
        conn,
        &FetchArticle {
            article_id: params.article_id,
            me: params.me.to_owned(),
        },
    );
    let _ = Favorite::unfavorite(
        conn,
        &UnfavoriteAction {
            user_id: params.me.id,
            article_id: params.article_id,
        },
    );
    item
}
