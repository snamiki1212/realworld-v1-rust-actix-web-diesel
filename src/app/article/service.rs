use crate::app::article::model::{Article, NewArticle, UpdateArticle};
use crate::app::follow::model::Follow;
use crate::app::profile;
use crate::app::profile::model::Profile;
use crate::app::profile::service::{fetch_profile_by_id, FetchProfileById};
use crate::app::tag::model::{NewTag, Tag};
use crate::app::user::model::User;
use crate::schema::articles::dsl::*;
use crate::schema::{articles, tags, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub struct CreateArticleSerivce {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
    pub me: User,
}
pub fn create(conn: &PgConnection, params: &CreateArticleSerivce) -> (Article, Profile, Vec<Tag>) {
    let article = Article::create(
        &conn,
        &NewArticle {
            author_id: params.author_id,
            slug: params.slug.to_owned(),
            title: params.title.to_owned(),
            description: params.description.to_owned(),
            body: params.body.to_owned(),
        },
    );
    let tag_list = create_tag_list(&conn, &params.tag_list, &article);
    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: params.me.to_owned(),
            id: article.author_id,
        },
    );
    (article, profile, tag_list)
}

fn create_tag_list(
    conn: &PgConnection,
    tag_list: &Option<Vec<String>>,
    article: &Article,
) -> Vec<Tag> {
    tag_list
        .as_ref()
        .map(|tag_list| {
            let records = tag_list
                .iter()
                .map(|tag| NewTag {
                    name: &tag,
                    article_id: &article.id,
                })
                .collect();
            Tag::create_list(&conn, records)
        })
        .unwrap_or(vec![])
}

pub fn fetch_articles_count(conn: &PgConnection) -> i64 {
    let articles_count = articles
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)
        .expect("couldn't fetch articles count.");
    articles_count
}

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
    pub me: User,
}

pub fn fetch_articles_list(
    conn: &PgConnection,
    params: FetchArticlesList,
) -> Vec<((Article, Profile), Vec<Tag>)> {
    use diesel::prelude::*;
    let article_and_user_list = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let tagged_article_ids = tags::table
                .filter(tags::name.eq(tag_name))
                .select(tags::article_id)
                .load::<Uuid>(conn)
                .expect("could not fetch tagged article ids.");
            query = query.filter(articles::id.eq_any(tagged_article_ids));
        }

        if let Some(author_name) = &params.author {
            let article_ids_by_author = users::table
                .inner_join(articles::table)
                .filter(users::username.eq(author_name))
                .select(articles::id)
                .load::<Uuid>(conn)
                .expect("could not fetch authors id.");
            query = query.filter(articles::id.eq_any(article_ids_by_author));
        }

        if let Some(favorited) = &params.favorited {
            // TODO:
            println!("==>favorited");
        }

        query
            .offset(params.offset)
            .limit(params.limit)
            .load::<(Article, User)>(conn)
            .expect("couldn't fetch articles list.")
    };

    let tags_list = {
        let articles_list = article_and_user_list
            .clone() // TODO: avoid clone
            .into_iter()
            .map(|(article, _)| article)
            .collect::<Vec<_>>();

        let tags_list = Tag::belonging_to(&articles_list)
            .load::<Tag>(conn)
            .expect("could not fetch tags list.");

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
            .filter(follows::follower_id.eq(params.me.id))
            .filter(follows::followee_id.eq_any(user_ids_list))
            .get_results::<Follow>(conn)
            .expect("could not fetch follow.");

        let follows_list = follows_list.into_iter();
        let article_and_profile_list = article_and_user_list
            .into_iter()
            .map(|(article, user)| {
                let following = follows_list.clone().any(|item| item.followee_id == user.id);
                let profile = Profile {
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                    following: following.to_owned(),
                };
                (article, profile)
            })
            .collect::<Vec<_>>();

        article_and_profile_list
    };

    let articles_list = article_and_profile_list
        .into_iter()
        .zip(tags_list)
        .collect::<Vec<_>>();

    articles_list
}

pub struct FetchArticle {
    pub article_id: Uuid,
    pub me: User,
}
pub fn fetch_article(conn: &PgConnection, params: &FetchArticle) -> (Article, Profile, Vec<Tag>) {
    use diesel::prelude::*;
    let FetchArticle { article_id, me } = params;
    let (article, author) = articles
        .inner_join(users::table)
        .filter(articles::id.eq(article_id))
        .get_result::<(Article, User)>(conn)
        .expect("could not fetch article by id.");

    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: me.to_owned(),
            id: author.id,
        },
    );

    let tags_list = Tag::belonging_to(&article)
        .load::<Tag>(conn)
        .expect("could not fetch tags list.");

    (article, profile, tags_list)
}

use crate::schema::articles::dsl::*;
use crate::schema::follows;
use crate::schema::follows::dsl::*;
pub struct FetchFollowedArticlesSerivce {
    pub me: User,
    pub offset: i64,
    pub limit: i64,
}
pub fn fetch_following_articles(
    conn: &PgConnection,
    params: &FetchFollowedArticlesSerivce,
) -> (Vec<((Article, User), Vec<Tag>)>, i64) {
    let query = {
        let following_user_ids = follows
            .filter(follows::follower_id.eq(params.me.id))
            .select(follows::followee_id)
            .get_results::<Uuid>(conn)
            .expect("could not fetch following uesrs.");

        articles.filter(articles::author_id.eq_any(following_user_ids))
    };

    let articles_list = {
        let article_and_user_list = query
            .to_owned()
            .inner_join(users::table)
            .limit(params.limit)
            .offset(params.offset)
            .order(articles::created_at.desc())
            .get_results::<(Article, User)>(conn)
            .expect("could not fetch following articles.");

        let articles_list = article_and_user_list
            .clone() // TODO: avoid clone
            .into_iter()
            .map(|(article, _)| article)
            .collect::<Vec<_>>();

        let tags_list = Tag::belonging_to(&articles_list)
            .load::<Tag>(conn)
            .expect("could not fetch tags list.");

        let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);

        let articles_list = article_and_user_list
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>();

        articles_list
    };

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)
        .expect("couldn't fetch articles count.");

    (articles_list, articles_count)
}

pub struct UpdateArticleService {
    pub me: User,
    pub article_id: Uuid,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
pub fn update_article(
    conn: &PgConnection,
    params: &UpdateArticleService,
) -> (Article, Profile, Vec<Tag>) {
    let article = Article::update(
        &conn,
        &params.article_id,
        &UpdateArticle {
            slug: params.slug.to_owned(),
            title: params.title.to_owned(),
            description: params.description.to_owned(),
            body: params.body.to_owned(),
        },
    );
    let tag_list = Tag::fetch_list_by_article_id(&conn, params.article_id);
    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: params.me.to_owned(),
            id: article.author_id,
        },
    );
    (article, profile, tag_list)
}
