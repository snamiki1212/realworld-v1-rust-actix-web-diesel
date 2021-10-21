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
    pub bio: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn signup<'a>(
        conn: &PgConnection,
        // email: &'a str,
        // bio: &'a str,
        // image: &'a Option(&str),
    ) -> Result<User, Error> {
        // use crate::schema;
        use diesel::prelude::*;
        // use schema::users;
        // use schema::users::dsl::*;

        // let now = std::time::SystemTime::now();

        let action = SignupUser {
            email: "email",
            bio: "bio",
            // image: Some("this is image"),
            // updated_at: NaiveDateTime::new(now),
        };
        let val = diesel::insert_into(users::table).values(&action);
        let result = val.get_result::<User>(conn).expect("Error saving user");
        Ok(result)
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub bio: &'a str,
    // pub image: Option<&'a str>,
    // pub updated_at: NaiveDateTime,
}
