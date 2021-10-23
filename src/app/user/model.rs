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

#[derive(Queryable, Serialize, Deserialize, Debug)]
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

    pub fn generate_token(&self) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        token::generate(now).expect("could not encode jwt.")
        // let payload = TokenPayload {
        //     iat: now,
        //     exp: now + ONE_DAY,
        // };

        // // TODO: move /utils/token
        // jsonwebtoken::encode(
        //     &Header::default(),
        //     &payload,
        //     &EncodingKey::from_secret(&KEY),
        // )
        // .expect("could not encode jwt.")
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
