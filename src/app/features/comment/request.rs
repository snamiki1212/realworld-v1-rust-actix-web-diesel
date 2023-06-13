use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub comment: InnerComment,
}

#[derive(Serialize, Deserialize)]
pub struct InnerComment {
    pub body: String,
}
