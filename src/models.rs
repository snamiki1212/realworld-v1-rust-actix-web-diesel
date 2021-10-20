use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
