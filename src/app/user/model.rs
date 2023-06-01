use crate::app::follow::model::{CreateFollow, DeleteFollow, Follow};
use crate::app::profile::model::Profile;
use crate::error::AppError;
use crate::schema::users;
use crate::utils::{hasher, token};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone)]
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
        let user = users::table
            .filter(users::email.eq(email))
            .limit(1)
            .first::<User>(conn)?;
        hasher::verify(naive_password, &user.password)?;
        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn find(conn: &mut PgConnection, id: Uuid) -> Result<Self, AppError> {
        let user = users::table.find(id).first(conn)?;
        Ok(user)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: Uuid,
        changeset: UpdateUser,
    ) -> Result<Self, AppError> {
        let target = users::table.filter(users::id.eq(user_id));
        let user = diesel::update(target)
            .set(changeset)
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<Self, AppError> {
        let user = users::table
            .filter(users::username.eq(username))
            .limit(1)
            .first::<User>(conn)?;
        Ok(user)
    }

    pub fn follow(&self, conn: &mut PgConnection, username: &str) -> Result<Profile, AppError> {
        let followee = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)?;

        Follow::create(
            conn,
            &CreateFollow {
                follower_id: self.id,
                followee_id: followee.id,
            },
        )?;

        Ok(Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following: true,
        })
    }

    pub fn unfollow(&self, conn: &mut PgConnection, username: &str) -> Result<Profile, AppError> {
        let followee = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)?;

        Follow::delete(
            conn,
            &DeleteFollow {
                followee_id: followee.id,
                follower_id: self.id,
            },
        )?;

        Ok(Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following: false,
        })
    }

    pub fn is_following(&self, conn: &mut PgConnection, followee_id: &Uuid) -> bool {
        use crate::schema::follows;
        let follow = follows::table
            .filter(follows::followee_id.eq(followee_id))
            .filter(follows::follower_id.eq(self.id))
            .get_result::<Follow>(conn);
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
        let favorited_article_ids = favorites::table
            .filter(favorites::user_id.eq(self.id))
            .select(favorites::article_id)
            .get_results::<Uuid>(conn)?;
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
