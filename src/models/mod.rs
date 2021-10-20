use super::schema::tags;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub name: &'a str,
}
