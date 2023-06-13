use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::schema::follows;
use chrono::NaiveDateTime;
use diesel::dsl::Eq;
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

type WithFollowee<T> = Eq<follows::followee_id, T>;
type WithFollower<T> = Eq<follows::follower_id, T>;

impl Follow {
    pub fn with_followee(followee_id: &Uuid) -> WithFollowee<&Uuid> {
        follows::followee_id.eq(followee_id)
    }

    pub fn with_follower(follower_id: &Uuid) -> WithFollower<&Uuid> {
        follows::follower_id.eq(follower_id)
    }
}

impl Follow {
    pub fn create(conn: &mut PgConnection, params: &CreateFollow) -> Result<(), AppError> {
        diesel::insert_into(follows::table)
            .values(params)
            .execute(conn)?;
        Ok(())
    }

    pub fn delete(conn: &mut PgConnection, params: &DeleteFollow) -> Result<(), AppError> {
        let t = follows::table
            .filter(Follow::with_followee(&params.followee_id))
            .filter(Follow::with_follower(&params.follower_id));
        diesel::delete(t).execute(conn)?;
        Ok(())
    }

    pub fn fetch_folowee_ids_by_follower_id(
        conn: &mut PgConnection,
        follower_id: &Uuid,
    ) -> Result<Vec<Uuid>, AppError> {
        let t = follows::table
            .filter(Follow::with_follower(follower_id))
            .select(follows::followee_id);
        let result = t.get_results::<Uuid>(conn)?;
        Ok(result)
    }
}

#[derive(Insertable)]
#[diesel(table_name = follows)]
pub struct CreateFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}

pub struct DeleteFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}
