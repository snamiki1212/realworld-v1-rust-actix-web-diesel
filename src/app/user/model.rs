use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
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

#[derive(Deserialize, Serialize)]
pub struct TokenPayload {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
}

static KEY: [u8; 16] = *include_bytes!("../../../secret.key"); // TODO:
static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

impl User {
    pub fn signup<'a>(
        conn: &PgConnection,
        email: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<(User, Token), Error> {
        use diesel::prelude::*;

        let hashed_password = hash(password, DEFAULT_COST).expect("could not hash password.");

        let record = SignupUser {
            email: email,
            username: username,
            password: &hashed_password,
        };
        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)
            .expect("Error saving user");

        let token = user.generate_token();
        let result = (user, token);
        Ok(result)
    }

    pub fn generate_token(&self) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = TokenPayload {
            iat: now,
            exp: now + ONE_DAY,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .unwrap()
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
