use crate::app::user::model::User;
use crate::schema::{follows, users};
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

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Associations)]
#[belongs_to(User, foreign_key = "followee_id", foreign_key = "follower_id")]
#[table_name = "follows"]
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
