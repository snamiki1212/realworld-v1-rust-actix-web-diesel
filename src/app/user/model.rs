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
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn signup<'a>(
        conn: &PgConnection,
        email: &'a str,
        // bio: &'a str,
        // image: &'a Option(&str),
    ) -> Result<User, Error> {
        // use crate::schema;
        use diesel::prelude::*;
        // use schema::users;
        // use schema::users::dsl::*;

        // let now = std::time::SystemTime::now();
        // let image = "1234";
        // let image = if image.len() % 2 == 0 {
        //     Some(image)
        // } else {
        //     None
        // };

        let record = SignupUser {
            email: email,
            // bio: bio,
            // image: image,
            // updated_at: NaiveDateTime::new(now),
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
    // pub bio: &'a str,
    // pub image: Option<&'a str>,
    // pub updated_at: NaiveDateTime,
}
