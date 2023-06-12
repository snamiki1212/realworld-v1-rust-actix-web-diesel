use crate::appv2::features::article::entities::Article;
use crate::appv2::features::favorite::entities::{Favorite, FavoriteInfo};
use crate::appv2::features::follow::entities::Follow;
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::Tag;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::schema::articles::dsl::*;
use crate::schema::{articles, tags, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile, FavoriteInfo);
pub type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;
pub type FetchArticlesListResult = (ArticlesList, ArticlesCount);
pub fn fetch_articles_list(
    conn: &mut PgConnection,
    params: FetchArticlesList,
) -> Result<FetchArticlesListResult, AppError> {
    use diesel::prelude::*;

    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let ids = Tag::fetch_article_ids_by_name(conn, tag_name)
                .expect("could not fetch tagged article ids."); // TODO: use ? or error handling
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(author_name) = &params.author {
            let ids = Article::fetch_ids_by_author_name(conn, author_name)
                .expect("could not fetch authors id."); // TODO: use ? or error handling
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(username) = &params.favorited {
            let ids = Favorite::fetch_favorited_article_ids_by_username(conn, username)
                .expect("could not fetch favorited articles id."); // TODO: use ? or error handling

            query = query.filter(articles::id.eq_any(ids));
        }

        query
    };
    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    let result = {
        let query = {
            let mut query = articles::table.inner_join(users::table).into_boxed();

            if let Some(tag_name) = &params.tag {
                let ids = Tag::fetch_article_ids_by_name(conn, tag_name)
                    .expect("could not fetch tagged article ids."); // TODO: use ? or error handling
                query = query.filter(articles::id.eq_any(ids));
            }

            if let Some(author_name) = &params.author {
                let ids = Article::fetch_ids_by_author_name(conn, author_name)
                    .expect("could not fetch authors id."); // TODO: use ? or error handling
                query = query.filter(articles::id.eq_any(ids));
            }

            if let Some(username) = &params.favorited {
                let ids = Favorite::fetch_favorited_article_ids_by_username(conn, username)
                    .expect("could not fetch favorited articles id."); // TODO: use ? or error handling

                query = query.filter(articles::id.eq_any(ids));
            }

            query
        };
        let article_and_user_list =
            query
                .offset(params.offset)
                .limit(params.limit)
                .load::<(Article, User)>(conn)?;

        let tags_list = {
            let articles_list = article_and_user_list
                .clone()
                .into_iter()
                .map(|(article, _)| article)
                .collect::<Vec<_>>();
            let tags_list = Tag::belonging_to(&articles_list)
                .order(tags::name.asc())
                .load::<Tag>(conn)?;
            let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);
            tags_list
        };

        let favorites_count_list = {
            let list: Result<Vec<_>, _> = article_and_user_list
                .clone()
                .into_iter()
                .map(|(article, _)| article.fetch_favorites_count(conn))
                .collect();

            list?
        };

        article_and_user_list
            .into_iter()
            .zip(favorites_count_list)
            .map(|((article, user), favorites_count)| {
                (
                    article,
                    Profile {
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                        following: false, // NOTE: because not authz
                    },
                    FavoriteInfo {
                        is_favorited: false, // NOTE: because not authz
                        favorites_count,
                    },
                )
            })
            .zip(tags_list)
            .collect::<Vec<_>>()
    };

    Ok((result, articles_count))
}

use crate::schema::follows;
