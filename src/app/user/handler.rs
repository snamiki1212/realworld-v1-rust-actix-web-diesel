// use super::model::User;
use crate::schema::users;
// use crate::utils::db::DbPool;
// use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct SignupRes {
    pub user: SignupResUser,
}

pub struct SignupResUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
