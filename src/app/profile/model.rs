use crate::app::user::model::User;
use crate::schema::follows;
use crate::schema::follows::dsl::*;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl Profile {
    pub fn find_by_name(conn: &PgConnection, username: &str) -> Result<Profile> {
        let user = User::find_by_username(conn, username)?;
        let profile = Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following: true, // TODO:
        };
        Ok(profile)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Follow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "follows"]
pub struct NewFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}
