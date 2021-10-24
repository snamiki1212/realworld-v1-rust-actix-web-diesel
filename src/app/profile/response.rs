use crate::app::profile::model::Profile as ProfileModel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProfileResponse {
    pub profile: ProfileContent,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProfileContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl ProfileResponse {
    pub fn from(profile_model: ProfileModel) -> Self {
        let inner = ProfileContent {
            username: profile_model.username,
            bio: profile_model.bio,
            image: profile_model.image,
            following: profile_model.following,
        };
        ProfileResponse { profile: inner }
    }
}
