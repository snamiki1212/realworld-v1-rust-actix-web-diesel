use super::super::entities::profile::Profile as ProfileModel;
use serde::{Deserialize, Serialize};
use std::convert::From;

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

impl From<ProfileModel> for ProfileResponse {
    fn from(profile_model: ProfileModel) -> Self {
        let profile = ProfileContent {
            username: profile_model.username,
            bio: profile_model.bio,
            image: profile_model.image,
            following: profile_model.following,
        };
        ProfileResponse { profile }
    }
}

pub struct ProfilePresenter {}
impl ProfilePresenter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn complete(&self, output: ProfileModel) -> ProfileResponse {
        ProfileResponse::from(output)
    }
}
