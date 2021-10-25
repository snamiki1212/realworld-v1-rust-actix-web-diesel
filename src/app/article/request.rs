use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub article: ArticeContent,
}

#[derive(Deserialize, Serialize)]
pub struct ArticeContent {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Option<Vec<String>>,
}
