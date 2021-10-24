use crate::app::profile::model::Profile;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::utils::token;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
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
    ) -> Result<(User, Token)> {
        use diesel::prelude::*;
        let hashed_password = Self::hash_password(naive_password);

        let record = SignupUser {
            email: _email,
            username: _username,
            password: &hashed_password,
        };
        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        let token = user.generate_token();
        let result = (user, token);
        Ok(result)
    }

    pub fn signin(
        conn: &PgConnection,
        _email: &str,
        naive_password: &str,
    ) -> Result<(User, Token)> {
        let user = users
            .filter(email.eq(_email))
            .limit(1)
            .first::<User>(conn)?;
        verify(&naive_password, &user.password)?;

        let token = user.generate_token();
        let result = (user, token);
        Ok(result)
    }

    fn hash_password(naive_pw: &str) -> String {
        hash(&naive_pw, DEFAULT_COST).expect("could not hash password.")
    }

    pub fn find_by_id(conn: &PgConnection, _id: Uuid) -> Self {
        users
            .find(_id)
            .first(conn)
            .expect("could not find user by id.")
    }

    pub fn update(conn: &PgConnection, user_id: Uuid, changeset: UpdatableUser) -> Result<Self> {
        let target = users.filter(id.eq(user_id));
        let user = diesel::update(target)
            .set(changeset)
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn find_by_username(conn: &PgConnection, _username: &str) -> Result<Self> {
        let user = users
            .filter(username.eq(_username))
            .limit(1)
            .first::<User>(conn)
            .expect("could not find user by username");
        Ok(user)
    }

    pub fn follow(&self, conn: &PgConnection, _username: &str) -> Result<Profile> {
        let followee = users
            .filter(username.eq(_username))
            .first::<User>(conn)
            .expect("could not find user by name.");

        {
            use crate::app::profile::model::NewFollow;
            use crate::schema::follows::dsl::*;
            diesel::insert_into(follows)
                .values(&NewFollow {
                    follower_id: self.id,
                    followee_id: followee.id,
                })
                .execute(conn)
                .expect("couldn't insert follow.");
        };
        let profile = Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following: true,
        };
        Ok(profile)
    }

    pub fn unfollow(&self, conn: &PgConnection, _username: &str) -> Result<Profile> {
        let followee = users
            .filter(username.eq(_username))
            .first::<User>(conn)
            .expect("could not find user by name.");

        {
            use crate::schema::follows::dsl::*;
            diesel::delete(
                follows
                    .filter(followee_id.eq(followee.id))
                    .filter(follower_id.eq(self.id)),
            )
            .execute(conn)
            .expect("couldn't delete follow.");
        };
        let profile = Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following: false,
        };
        Ok(profile)
    }
}

impl User {
    pub fn generate_token(&self) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        token::generate(self.id, now).expect("could not encode jwt.")
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
pub struct UpdatableUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}
