use crate::app::user::model::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {
    pub profile: ProfileContent,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProfileContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
