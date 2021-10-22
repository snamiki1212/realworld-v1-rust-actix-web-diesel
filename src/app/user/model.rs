use crate::schema::users;
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
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

impl User {
    pub fn signup<'a>(
        conn: &PgConnection,
        email: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<User, Error> {
        use diesel::prelude::*;

        let record = SignupUser {
            email: email,
            username: username,
            password: password,
        };
        let result = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)
            .expect("Error saving user");
        Ok(result)
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
