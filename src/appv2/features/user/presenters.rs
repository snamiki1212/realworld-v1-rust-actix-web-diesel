use crate::appv2::features::user::entities::User;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserResponse {
    pub user: AuthUser,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        // REF: https://gothinkster.github.io/realworld/docs/specs/backend-specs/api-response-format/#users-for-authentication
        Self {
            user: AuthUser {
                email: user.email,
                token,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone)]
pub struct UserPresenter {}
impl UserPresenter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn from_user_and_token(&self, user: User, token: String) -> HttpResponse {
        let res_model = UserResponse::from((user, token));
        HttpResponse::Ok().json(res_model)
    }
}
