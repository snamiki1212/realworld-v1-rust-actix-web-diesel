use super::model::Tag;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TagsResponse {
    // SPEC: https://gothinkster.github.io/realworld/docs/specs/backend-specs/endpoints#registration
    pub tags: Vec<String>,
}

impl std::convert::From<Vec<Tag>> for TagsResponse {
    fn from(tags: Vec<Tag>) -> Self {
        let list = tags.iter().map(move |tag| tag.name.clone()).collect();
        TagsResponse { tags: list }
    }
}
