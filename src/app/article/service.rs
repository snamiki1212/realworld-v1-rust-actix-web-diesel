use crate::app::article::model::{Article, CreateArticle, UpdateArticle};
use crate::app::favorite::model::FavoriteInfo;
use crate::app::follow::model::Follow;
use crate::app::profile;
use crate::app::profile::model::Profile;
use crate::app::profile::service::FetchProfileById;
use crate::app::tag::model::{CreateTag, Tag};
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::articles::dsl::*;
use crate::schema::{articles, favorites, tags, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub struct CreateArticleSerivce {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
    pub current_user: User,
}
pub fn create(
    conn: &PgConnection,
    params: &CreateArticleSerivce,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let article = Article::create(
        conn,
        &CreateArticle {
            author_id: params.current_user.id,
            slug: params.slug.clone(),
            title: params.title.clone(),
            description: params.description.clone(),
            body: params.body.clone(),
        },
    )?;
    let tag_list = create_tag_list(conn, &params.tag_list, &article)?;
    let profile = profile::service::fetch_profile_by_id(
        conn,
        &FetchProfileById {
            user: params.current_user.to_owned(),
            id: article.author_id,
        },
    )?;

    let favorite_info = {
        let is_favorited = article.is_favorited_by_user_id(conn, &params.current_user.id)?;
        let favorites_count = article.fetch_favorites_count(conn)?;
        FavoriteInfo {
            is_favorited,
            favorites_count,
        }
    };

    Ok((article, profile, favorite_info, tag_list))
}

fn create_tag_list(
    conn: &PgConnection,
    tag_list: &Option<Vec<String>>,
    article: &Article,
) -> Result<Vec<Tag>, AppError> {
    let list = tag_list
        .as_ref()
        .map(|tag_list| {
            let records = tag_list
                .iter()
                .map(|tag| CreateTag {
                    name: tag,
                    article_id: &article.id,
                })
                .collect();
            Tag::create_list(conn, records)
        })
        .unwrap_or_else(|| Ok(vec![]));
    list
}

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile, FavoriteInfo);
type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;
pub fn fetch_articles_list(
    conn: &PgConnection,
    params: FetchArticlesList,
) -> Result<(ArticlesList, ArticlesCount), AppError> {
    use diesel::prelude::*;
    let query = || {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let tagged_article_ids = tags::table
                .filter(tags::name.eq(tag_name))
                .select(tags::article_id)
                .load::<Uuid>(conn)
                .expect("could not fetch tagged article ids."); // TODO: use ? or error handling
            query = query.filter(articles::id.eq_any(tagged_article_ids));
        }

        if let Some(author_name) = &params.author {
            let article_ids_by_author = users::table
                .inner_join(articles::table)
                .filter(users::username.eq(author_name))
                .select(articles::id)
                .load::<Uuid>(conn)
                .expect("could not fetch authors id."); // TODO: use ? or error handling
            query = query.filter(articles::id.eq_any(article_ids_by_author));
        }

        if let Some(favorited_username) = &params.favorited {
            let favorited_article_ids = favorites::table
                .inner_join(users::table)
                .filter(users::username.eq(favorited_username))
                .select(favorites::article_id)
                .load::<Uuid>(conn)
                .expect("could not fetch favorited articles id."); // TODO: use ? or error handling
            query = query.filter(articles::id.eq_any(favorited_article_ids));
        }

        query
    };

    let articles_count = query()
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    let articles_list = {
        let article_and_user_list =
            query()
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
            let favorites_count_list: Result<Vec<_>, _> = article_and_user_list
                .clone()
                .into_iter()
                .map(|(article, _)| article.fetch_favorites_count(conn))
                .collect();

            favorites_count_list?
        };

        let article_and_profile_list = {
            article_and_user_list
                .into_iter()
                .map(|(article, user)| {
                    let profile = Profile {
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                        following: false, // NOTE: because not authz
                    };
                    let is_favorited = false; // NOTE: because not authz
                    (article, profile, is_favorited)
                })
                .zip(favorites_count_list)
                .map(|((article, profile, is_favorited), favorites_count)| {
                    (
                        article,
                        profile,
                        FavoriteInfo {
                            is_favorited,
                            favorites_count,
                        },
                    )
                })
                .collect::<Vec<_>>()
        };

        article_and_profile_list
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>()
    };

    Ok((articles_list, articles_count))
}

pub struct FetchArticle {
    pub article_id: Uuid,
    pub current_user: User,
}
pub fn fetch_article(
    conn: &PgConnection,
    FetchArticle { article_id, current_user: me }: &FetchArticle,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let (article, author) = articles
        .inner_join(users::table)
        .filter(articles::id.eq(article_id))
        .get_result::<(Article, User)>(conn)?;

    let profile = profile::service::fetch_profile_by_id(
        conn,
        &FetchProfileById {
            user: me.to_owned(),
            id: author.id,
        },
    )?;

    let favorite_info = {
        let is_favorited = article.is_favorited_by_user_id(conn, &me.id)?;
        let favorites_count = article.fetch_favorites_count(conn)?;
        FavoriteInfo {
            is_favorited,
            favorites_count,
        }
    };

    let tags_list = Tag::belonging_to(&article).load::<Tag>(conn)?;

    Ok((article, profile, favorite_info, tags_list))
}

pub struct FetchArticleBySlug {
    pub article_title_slug: String,
}
pub fn fetch_article_by_slug(
    conn: &PgConnection,
    params: &FetchArticleBySlug,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    use diesel::prelude::*;
    let FetchArticleBySlug { article_title_slug } = params;
    let (article, author) = articles
        .inner_join(users::table)
        .filter(articles::slug.eq(article_title_slug))
        .get_result::<(Article, User)>(conn)?;

    let profile = profile::service::fetch_profile_by_id(
        conn,
        &FetchProfileById {
            user: author.to_owned(),
            id: author.id,
        },
    )?;

    let tags_list = Tag::belonging_to(&article).load::<Tag>(conn)?;

    let favorite_info = {
        let is_favorited = article.is_favorited_by_user_id(conn, &author.id)?;
        let favorites_count = article.fetch_favorites_count(conn)?;
        FavoriteInfo {
            is_favorited,
            favorites_count,
        }
    };

    Ok((article, profile, favorite_info, tags_list))
}

use crate::schema::follows;
use crate::schema::follows::dsl::*;
pub struct FetchFollowedArticlesSerivce {
    pub current_user: User,
    pub offset: i64,
    pub limit: i64,
}
pub fn fetch_following_articles(
    conn: &PgConnection,
    params: &FetchFollowedArticlesSerivce,
) -> Result<(ArticlesList, ArticlesCount), AppError> {
    let query = {
        let following_user_ids = follows
            .filter(follows::follower_id.eq(params.current_user.id))
            .select(follows::followee_id)
            .get_results::<Uuid>(conn)?;

        articles.filter(articles::author_id.eq_any(following_user_ids))
    };

    let articles_list = {
        let article_and_user_list = query
            .to_owned()
            .inner_join(users::table)
            .limit(params.limit)
            .offset(params.offset)
            .order(articles::created_at.desc())
            .get_results::<(Article, User)>(conn)?;

        let tags_list = {
            let articles_list = article_and_user_list
                .clone() // TODO: avoid clone
                .into_iter()
                .map(|(article, _)| article)
                .collect::<Vec<_>>();

            let tags_list = Tag::belonging_to(&articles_list).load::<Tag>(conn)?;
            let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);
            tags_list
        };

        let article_and_profile_list = {
            let user_ids_list = article_and_user_list
                .clone() // TODO: avoid clone
                .into_iter()
                .map(|(_, user)| user.id)
                .collect::<Vec<_>>();

            let follows_list = follows::table
                .filter(follows::follower_id.eq(params.current_user.id))
                .filter(follows::followee_id.eq_any(user_ids_list))
                .get_results::<Follow>(conn)?;

            let favorites_count_list = {
                let favorites_count_list: Result<Vec<_>, _> = article_and_user_list
                    .clone()
                    .into_iter()
                    .map(|(article, _)| article.fetch_favorites_count(conn))
                    .collect();

                favorites_count_list?
            };

            let favorited_article_ids = params.current_user.fetch_favorited_article_ids(conn)?;

            let is_favorited_by_me = |article: &Article| {
                favorited_article_ids
                    .iter()
                    .copied()
                    .any(|_id| _id == article.id)
            };

            let follows_list = follows_list.into_iter();
            article_and_user_list
                .into_iter()
                .map(|(article, user)| {
                    let following = follows_list.clone().any(|item| item.followee_id == user.id);
                    let profile = Profile {
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                        following: following.to_owned(),
                    };
                    let is_favorited = is_favorited_by_me(&article);
                    (article, profile, is_favorited)
                })
                .zip(favorites_count_list)
                .map(|((article, profile, is_favorited), favorites_count)| {
                    (
                        article,
                        profile,
                        FavoriteInfo {
                            is_favorited,
                            favorites_count,
                        },
                    )
                })
                .collect::<Vec<_>>()
        };

        article_and_profile_list
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>()
    };

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    Ok((articles_list, articles_count))
}

pub struct UpdateArticleService {
    pub current_user: User,
    pub article_title_slug: String,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
pub fn update_article(
    conn: &PgConnection,
    params: &UpdateArticleService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let article = Article::update(
        conn,
        &params.article_title_slug,
        &params.current_user.id,
        &UpdateArticle {
            slug: params.slug.to_owned(),
            title: params.title.to_owned(),
            description: params.description.to_owned(),
            body: params.body.to_owned(),
        },
    )?;

    let tag_list = Tag::fetch_list_by_article_id(conn, article.id)?;

    let profile = profile::service::fetch_profile_by_id(
        conn,
        &FetchProfileById {
            user: params.current_user.to_owned(),
            id: article.author_id,
        },
    )?;

    let favorite_info = {
        let is_favorited = article.is_favorited_by_user_id(conn, &params.current_user.id)?;
        let favorites_count = article.fetch_favorites_count(conn)?;
        FavoriteInfo {
            is_favorited,
            favorites_count,
        }
    };

    Ok((article, profile, favorite_info, tag_list))
}
