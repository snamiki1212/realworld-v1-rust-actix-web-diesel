// use super::model::User;
use crate::schema::users;
// use crate::utils::db::DbPool;
// use actix_web::{get, post, put, web, HttpResponse, Responder};
use super::model::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupReq {
    pub user: SignupReqUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupReqUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupRes {
    pub user: SignupResUser,
}

impl SignupRes {
    pub fn from(user: User) -> SignupRes {
        SignupRes {
            user: SignupResUser {
                email: user.email,
                username: user.username,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupResUser {
    pub email: String,
    // pub token: String,
    pub username: String,
}
