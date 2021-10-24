use crate::app::user::model::User;
use anyhow::Result;
use diesel;
use diesel::pg::PgConnection;

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
            following: true,
        };
        Ok(profile)
    }
}
