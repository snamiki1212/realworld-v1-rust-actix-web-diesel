use crate::app::article::model::Article;
use crate::app::article::service::{fetch_article, FetchArticle};
use crate::app::favorite::model::{Favorite, FavorteAction};
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct FavoriteSerive {
    pub me: User,
    pub article_id: Uuid,
}

pub fn favorite(conn: &PgConnection, params: &FavoriteSerive) -> (Article, Profile, Vec<Tag>) {
    let _ = Favorite::favorite(
        conn,
        &FavorteAction {
            user_id: params.me.id,
            article_id: params.article_id,
        },
    );
    let list = fetch_article(
        conn,
        &FetchArticle {
            article_id: params.article_id,
            me: params.me.to_owned(),
        },
    );
    list
}
