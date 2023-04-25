use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::follows;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Associations, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = followee_id, foreign_key = follower_id))]
#[diesel(table_name = follows)]
pub struct Follow {
    pub followee_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Follow {
    pub fn create(conn: &mut PgConnection, params: &CreateFollow) -> Result<(), AppError> {
        diesel::insert_into(follows::table)
            .values(params)
            .execute(conn)?;
        Ok(())
    }

    pub fn delete(conn: &mut PgConnection, params: &DeleteFollow) -> Result<(), AppError> {
        diesel::delete(
            follows::table
                .filter(follows::followee_id.eq(params.followee_id))
                .filter(follows::follower_id.eq(params.follower_id)),
        )
        .execute(conn)?;
        Ok(())
    }

    pub fn fetch_folowee_ids_by_follower_id(
        conn: &mut PgConnection,
        follower_id: &Uuid,
    ) -> Result<Vec<Uuid>, AppError> {
        let result = follows::table
            .filter(follows::follower_id.eq(follower_id))
            .select(follows::followee_id)
            .get_results::<Uuid>(conn)?;
        Ok(result)
    }
}

#[derive(Insertable)]
#[table_name = "follows"]
pub struct CreateFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}

pub struct DeleteFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}
