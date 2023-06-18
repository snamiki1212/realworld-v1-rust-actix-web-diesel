use super::entities::{Article, CreateArticle, DeleteArticle, UpdateArticle};
use crate::app::features::favorite::entities::FavoriteInfo;
use crate::app::features::profile::entities::Profile;
use crate::app::features::tag::entities::{CreateTag, Tag};
use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;
use diesel::PgConnection;
use uuid::Uuid;

pub trait ArticleRepository: Send + Sync + 'static {
    fn fetch_articles(
        &self,
        params: FetchArticlesRepositoryInput,
    ) -> Result<(ArticlesList, ArticlesCount), AppError>;

    fn fetch_article_by_slug(
        &self,
        article_title_slug: String,
    ) -> Result<FetchArticleBySlugOutput, AppError>;

    fn create_article(
        &self,
        params: CreateArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError>;

    fn delete_article(&self, input: DeleteArticleRepositoryInput) -> Result<(), AppError>;

    fn update_article(
        &self,
        input: UpdateArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError>;

    fn fetch_article(
        &self,
        input: &FetchArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError>;

    fn fetch_following_articles(
        &self,
        params: &FetchFollowingArticlesRepositoryInput,
    ) -> Result<(ArticlesList, ArticlesCount), AppError>;
}
#[derive(Clone)]
pub struct ArticleRepositoryImpl {
    pool: DbPool,
}

impl ArticleRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn create_tag_list(
        conn: &mut PgConnection,
        tag_name_list: &Option<Vec<String>>,
        article_id: &Uuid,
    ) -> Result<Vec<Tag>, AppError> {
        let list = tag_name_list
            .as_ref()
            .map(|tag_name_list| {
                let records = tag_name_list
                    .iter()
                    .map(|name| CreateTag { name, article_id })
                    .collect();
                Tag::create_list(conn, records)
            })
            .unwrap_or_else(|| Ok(vec![]));
        list
    }
}

impl ArticleRepository for ArticleRepositoryImpl {
    fn fetch_articles(
        &self,
        params: FetchArticlesRepositoryInput,
    ) -> Result<(ArticlesList, ArticlesCount), AppError> {
        use crate::app::features::favorite::entities::Favorite;
        use crate::schema::{articles, tags, users};
        use diesel::prelude::*;
        // ====
        let conn = &mut self.pool.get()?;

        let query = {
            let mut query = articles::table.inner_join(users::table).into_boxed();

            if let Some(tag_name) = &params.tag {
                let ids = Tag::fetch_article_ids_by_name(conn, tag_name)
                    .expect("could not fetch tagged article ids."); // TODO: us e ? or error handling
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

    fn fetch_article_by_slug(
        &self,
        article_title_slug: String,
    ) -> Result<FetchArticleBySlugOutput, AppError> {
        let conn = &mut self.pool.get()?;

        let (article, author) = Article::fetch_by_slug_with_author(conn, &article_title_slug)?;

        let profile = author.fetch_profile(conn, &author.id)?;

        let tags_list = {
            use diesel::prelude::*;
            Tag::belonging_to(&article).load::<Tag>(conn)?
        };

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

    fn create_article(
        &self,
        params: CreateArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;

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

        let tag_list = Self::create_tag_list(conn, &params.tag_name_list, &article.id)?;

        let profile = params
            .current_user
            .fetch_profile(conn, &article.author_id)?;

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

    fn delete_article(&self, input: DeleteArticleRepositoryInput) -> Result<(), AppError> {
        let conn = &mut self.pool.get()?;
        Article::delete(
            conn,
            &DeleteArticle {
                slug: input.slug,
                author_id: input.author_id,
            },
        )
    }

    fn update_article(
        &self,
        input: UpdateArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;

        let article = Article::update(
            conn,
            &input.article_title_slug,
            &input.current_user.id,
            &UpdateArticle {
                slug: input.slug.to_owned(),
                title: input.title.to_owned(),
                description: input.description.to_owned(),
                body: input.body.to_owned(),
            },
        )?;

        let tag_list = Tag::fetch_by_article_id(conn, &article.id)?;

        let profile = input.current_user.fetch_profile(conn, &article.author_id)?;

        let favorite_info = {
            let is_favorited = article.is_favorited_by_user_id(conn, &input.current_user.id)?;
            let favorites_count = article.fetch_favorites_count(conn)?;
            FavoriteInfo {
                is_favorited,
                favorites_count,
            }
        };

        Ok((article, profile, favorite_info, tag_list))
    }

    fn fetch_article(
        &self,
        input: &FetchArticleRepositoryInput,
    ) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
        let conn = &mut self.pool.get()?;
        let (article, author) = Article::find_with_author(conn, &input.article_id)?;

        let profile = input.current_user.fetch_profile(conn, &author.id)?;

        let favorite_info = {
            let is_favorited = article.is_favorited_by_user_id(conn, &input.current_user.id)?;
            let favorites_count = article.fetch_favorites_count(conn)?;
            FavoriteInfo {
                is_favorited,
                favorites_count,
            }
        };

        let tags_list = {
            use diesel::prelude::*;
            Tag::belonging_to(&article).load::<Tag>(conn)?
        };

        Ok((article, profile, favorite_info, tags_list))
    }

    fn fetch_following_articles(
        &self,
        params: &FetchFollowingArticlesRepositoryInput,
    ) -> Result<(ArticlesList, ArticlesCount), AppError> {
        use crate::app::features::follow::entities::Follow;
        use crate::schema::articles::dsl::*;
        use crate::schema::follows;
        use crate::schema::{articles, users};
        use diesel::prelude::*;

        let conn = &mut self.pool.get()?;

        let create_query = {
            let ids = Follow::fetch_folowee_ids_by_follower_id(conn, &params.current_user.id)?;
            articles.filter(articles::author_id.eq_any(ids))
        };

        let articles_list = {
            let article_and_user_list = create_query
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

            let follows_list = {
                let user_ids_list = article_and_user_list
                    .clone() // TODO: avoid clone
                    .into_iter()
                    .map(|(_, user)| user.id)
                    .collect::<Vec<_>>();

                let list = follows::table
                    .filter(Follow::with_follower(&params.current_user.id))
                    .filter(follows::followee_id.eq_any(user_ids_list))
                    .get_results::<Follow>(conn)?;

                list.into_iter()
            };

            let favorites_count_list = {
                let list: Result<Vec<_>, _> = article_and_user_list
                    .clone()
                    .into_iter()
                    .map(|(article, _)| article.fetch_favorites_count(conn))
                    .collect();

                list?
            };

            let favorited_article_ids = params.current_user.fetch_favorited_article_ids(conn)?;
            let is_favorited_by_me = |article: &Article| {
                favorited_article_ids
                    .iter()
                    .copied()
                    .any(|_id| _id == article.id)
            };

            article_and_user_list
                .into_iter()
                .zip(favorites_count_list)
                .map(|((article, user), favorites_count)| {
                    let following = follows_list.clone().any(|item| item.followee_id == user.id);
                    let is_favorited = is_favorited_by_me(&article);
                    (
                        article,
                        Profile {
                            username: user.username,
                            bio: user.bio,
                            image: user.image,
                            following: following.to_owned(),
                        },
                        FavoriteInfo {
                            is_favorited,
                            favorites_count,
                        },
                    )
                })
                .zip(tags_list)
                .collect::<Vec<_>>()
        };

        let articles_count = create_query
            .select(diesel::dsl::count(articles::id))
            .first::<i64>(conn)?;

        Ok((articles_list, articles_count))
    }
}

pub struct CreateArticleRepositoryInput {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
    pub current_user: User,
}

pub struct DeleteArticleRepositoryInput {
    pub slug: String,
    pub author_id: Uuid,
}

pub struct UpdateArticleRepositoryInput {
    pub current_user: User,
    pub article_title_slug: String,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

pub type FetchArticleBySlugOutput = (Article, Profile, FavoriteInfo, Vec<Tag>);

pub struct FetchArticlesRepositoryInput {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

pub struct FetchArticleRepositoryInput {
    pub article_id: Uuid,
    pub current_user: User,
}

pub struct FetchFollowingArticlesRepositoryInput {
    pub current_user: User,
    pub offset: i64,
    pub limit: i64,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile, FavoriteInfo);
pub type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;
