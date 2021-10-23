use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Tag {
    pub fn list(conn: &PgConnection) -> Result<Vec<Tag>, Error> {
        use crate::schema;
        use diesel::prelude::*;
        use schema::tags::dsl::*;

        let list = tags
            // .filter(name.eq("react"))
            .limit(5)
            .load::<Tag>(conn);
        list
    }
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub name: &'a str,
}
