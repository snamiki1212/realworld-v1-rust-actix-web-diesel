use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub article: CreateArticleInner,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticleInner {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticleInner,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateArticleInner {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
