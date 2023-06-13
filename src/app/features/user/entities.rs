use crate::app::features::favorite::entities::Favorite;
use crate::app::features::follow::entities::Follow;
use crate::app::features::profile::entities::Profile;
use crate::error::AppError;
use crate::schema::users;
use crate::utils::{hasher, token};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::dsl::{AsSelect, Eq, Filter, Select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

type Token = String;

type All<DB> = Select<users::table, AsSelect<User, DB>>;
type WithUsername<T> = Eq<users::username, T>;
type WithEmail<T> = Eq<users::email, T>;
type ByUsername<DB, T> = Filter<All<DB>, WithUsername<T>>;
type ByEmail<DB, T> = Filter<All<DB>, WithEmail<T>>;

impl User {
    fn all<DB>() -> All<DB>
    where
        DB: Backend,
    {
        users::table.select(User::as_select())
    }

    pub fn with_username(username: &str) -> WithUsername<&str> {
        users::username.eq(username)
    }

    pub fn by_username<DB>(username: &str) -> ByUsername<DB, &str>
    where
        DB: Backend,
    {
        Self::all().filter(Self::with_username(username))
    }

    fn with_email(email: &str) -> WithEmail<&str> {
        users::email.eq(email)
    }

    fn by_email<DB>(email: &str) -> ByEmail<DB, &str>
    where
        DB: Backend,
    {
        Self::all().filter(Self::with_email(email))
    }
}

impl User {
    pub fn signup<'a>(
        conn: &mut PgConnection,
        email: &'a str,
        username: &'a str,
        naive_password: &'a str,
    ) -> Result<(User, Token), AppError> {
        use diesel::prelude::*;
        let hashed_password = hasher::hash_password(naive_password)?;

        let record = SignupUser {
            email,
            username,
            password: &hashed_password,
        };

        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn signin(
        conn: &mut PgConnection,
        email: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError> {
        let t = Self::by_email(email).limit(1);
        let user = t.first::<User>(conn)?;
        hasher::verify(naive_password, &user.password)?;
        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn find(conn: &mut PgConnection, id: Uuid) -> Result<Self, AppError> {
        let t = users::table.find(id);
        let user = t.first(conn)?;
        Ok(user)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: Uuid,
        changeset: UpdateUser,
    ) -> Result<Self, AppError> {
        let target = users::table.find(user_id);
        let user = diesel::update(target)
            .set(changeset)
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<Self, AppError> {
        let t = Self::by_username(username).limit(1);
        let user = t.first::<User>(conn)?;
        Ok(user)
    }

    pub fn is_following(&self, conn: &mut PgConnection, followee_id: &Uuid) -> bool {
        use crate::schema::follows;
        let t = follows::table
            .filter(Follow::with_followee(followee_id))
            .filter(Follow::with_follower(&self.id));
        let follow = t.get_result::<Follow>(conn);
        follow.is_ok()
    }
}

impl User {
    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let token = token::generate(self.id, now)?;
        Ok(token)
    }

    pub fn fetch_favorited_article_ids(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::favorites;
        let t = favorites::table
            .filter(Favorite::with_user_id(&self.id))
            .select(favorites::article_id);
        let favorited_article_ids = t.get_results::<Uuid>(conn)?;
        Ok(favorited_article_ids)
    }

    pub fn fetch_profile(
        &self,
        conn: &mut PgConnection,
        folowee_id: &Uuid,
    ) -> Result<Profile, AppError> {
        let is_following = &self.is_following(conn, folowee_id);
        let profile = Profile {
            username: self.username.to_owned(),
            bio: self.bio.to_owned(),
            image: self.image.to_owned(),
            following: is_following.to_owned(),
        };
        Ok(profile)
    }

    pub fn to_profile(&self, conn: &mut PgConnection, current_user: &Option<User>) -> Profile {
        let user = self;
        let following = match current_user {
            Some(current_user) => current_user.is_following(conn, &user.id),
            None => false,
        };

        Profile {
            username: user.username.to_owned(),
            bio: user.bio.to_owned(),
            image: user.image.to_owned(),
            following,
        }
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset, Debug, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}
