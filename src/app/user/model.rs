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

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone, Associations)]
#[table_name = "users"]
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
        conn: &PgConnection,
        _email: &'a str,
        _username: &'a str,
        naive_password: &'a str,
    ) -> Result<(User, Token), AppError> {
        use diesel::prelude::*;
        let hashed_password = hasher::hash_password(naive_password)?;

        let record = SignupUser {
            email: _email,
            username: _username,
            password: &hashed_password,
        };

        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn signin(
        conn: &PgConnection,
        _email: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError> {
        let user = users::table
            .filter(users::email.eq(_email))
            .limit(1)
            .first::<User>(conn)?;
        let _ = hasher::verify(&naive_password, &user.password)?;
        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn find(conn: &PgConnection, _id: Uuid) -> Result<Self, AppError> {
        let user = users::table.find(_id).first(conn)?;
        Ok(user)
    }

    pub fn update(
        conn: &PgConnection,
        user_id: Uuid,
        changeset: UpdateUser,
    ) -> Result<Self, AppError> {
        let target = users::table.filter(users::id.eq(user_id));
        let user = diesel::update(target)
            .set(changeset)
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn find_by_username(conn: &PgConnection, _username: &str) -> Result<Self, AppError> {
        let user = users::table
            .filter(users::username.eq(_username))
            .limit(1)
            .first::<User>(conn)?;
        Ok(user)
    }

    pub fn follow(&self, conn: &PgConnection, _username: &str) -> Result<Profile, AppError> {
        let followee = users::table
            .filter(users::username.eq(_username))
            .first::<User>(conn)?;

        let _ = Follow::create(
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

    pub fn unfollow(&self, conn: &PgConnection, _username: &str) -> Result<Profile, AppError> {
        let followee = users::table
            .filter(users::username.eq(_username))
            .first::<User>(conn)?;

        let _ = Follow::delete(
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

    pub fn is_following(&self, conn: &PgConnection, _followee_id: &Uuid) -> bool {
        use crate::schema::follows::dsl::*;
        let follow = follows
            .filter(followee_id.eq(_followee_id))
            .filter(follower_id.eq(self.id))
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

    pub fn fetch_favorited_article_ids(&self, conn: &PgConnection) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::favorites;
        let favorited_article_ids = favorites::table
            .filter(favorites::user_id.eq(self.id))
            .select(favorites::article_id)
            .get_results::<Uuid>(conn)?;
        Ok(favorited_article_ids)
    }

    pub fn fetch_profile(
        &self,
        conn: &PgConnection,
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
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset, Debug, Deserialize, Clone)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}
