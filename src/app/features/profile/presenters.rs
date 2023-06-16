use super::entities::Profile as ProfileModel;
use actix_web::HttpResponse;
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

pub trait ProfilePresenter: Send + Sync + 'static {
    fn to_json(&self, model: ProfileModel) -> HttpResponse;
}

#[derive(Clone)]
pub struct ProfilePresenterImpl {}
impl ProfilePresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl ProfilePresenter for ProfilePresenterImpl {
    fn to_json(&self, model: ProfileModel) -> HttpResponse {
        let res_model = ProfileResponse::from(model);
        HttpResponse::Ok().json(res_model)
    }
}
