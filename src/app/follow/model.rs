use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::follows;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Associations, Clone, Serialize, Deserialize)]
#[belongs_to(User, foreign_key = "followee_id", foreign_key = "follower_id")]
#[table_name = "follows"]
pub struct Follow {
    pub followee_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Follow {
    pub fn create_follow(conn: &PgConnection, params: &NewFollow) -> Result<(), AppError> {
        use diesel::prelude::*;
        let _ = diesel::insert_into(follows::table)
            .values(params)
            .execute(conn)?;
        Ok(())
    }

    pub fn delete_follow(conn: &PgConnection, params: &DeleteFollow) -> Result<(), AppError> {
        use crate::schema::follows::dsl::*;
        use diesel::prelude::*;
        let _ = diesel::delete(
            follows
                .filter(followee_id.eq(params.followee_id))
                .filter(follower_id.eq(params.follower_id)),
        )
        .execute(conn)?;
        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "follows"]
pub struct NewFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}

pub struct DeleteFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}
