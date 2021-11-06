use actix_web::{error::Error as ActixWebError, HttpRequest, HttpResponse};
use bcrypt::BcryptError;
use diesel::r2d2::{Error, PoolError};
use diesel::result::Error as DieselError;
use serde_json::Value as JsonValue;
use std::convert::From;
use thiserror::Error;
// use std::fmt::{self, Debug, Display};
// use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    // 401
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "An Error Occurred, Please Try Again!") // TODO:
//     }
// }

// impl From<AppError> for ActixWebError {
//     fn from(err: AppError) -> ActixWebError {
//         match err {
//             AppError::HogeError(str) => actix_web::error::ErrorNotFound("not found error"),
//             _ => actix_web::error::Error,
//         }
//     }
// }

impl From<AppError> for HttpResponse {
    fn from(err: AppError) -> HttpResponse {
        match err {
            AppError::InternalServerError => HttpResponse::Unauthorized().json("msg"),
            _ => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}

impl From<PoolError> for AppError {
    fn from(_err: PoolError) -> Self {
        AppError::InternalServerError
    }
}

impl From<BcryptError> for AppError {
    fn from(_err: BcryptError) -> Self {
        AppError::InternalServerError
    }
}

impl From<DieselError> for AppError {
    fn from(_error: DieselError) -> Self {
        AppError::InternalServerError
        // match error {
        //     DieselError::DatabaseError(kind, info) => {
        //         if let DatabaseErrorKind::UniqueViolation = kind {
        //             let message = info.details().unwrap_or_else(|| info.message()).to_string();
        //             return Error::UnprocessableEntity(json!({ "error": message }));
        //         }
        //         AppError::InternalServerError
        //     }
        //     DieselError::NotFound => {
        //         AppError::NotFound(json!({ "error": "requested record was not found" }))
        //     }
        //     _ => AppError::InternalServerError,
        // }
    }
}

// #[derive(Error, Debug)]
// pub struct MyError {
//     err: anyhow::Error,
// }
// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "An Error Occurred, Please Try Again!") // TODO:
//     }
// }
// impl actix_web::error::ResponseError for MyError {}

// impl From<anyhow::Error> for MyError {
//     fn from(err: anyhow::Error) -> MyError {
//         MyError { err }
//     }
// }

// impl From<anyhow::Error> for actix_web::error::Error {
//     fn from(err: anyhow::Error) -> actix_web::error::Error {
//         match err {
//             AppError::HogeError(str) => actix_web::error::ErrorNotFound(str),
//         }
//     }
// }

// impl actix_web::error::ResponseError for AppError {
//     fn error_response(&self) -> HttpResponse {
//         match *self {
//             AppError::HogeError(str) => HttpResponse::Unauthorized().json("not found error"),
//             _ => HttpResponse::Forbidden().json("forbidden"),
//         }
//     }
// }
