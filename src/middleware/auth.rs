use crate::app::user::model::User;
use crate::constants;
use crate::error::AppError;
use crate::middleware;
use crate::middleware::state::AppState;
use crate::utils::token;
use actix_service::{Service, Transform};
use actix_web::HttpMessage;
use actix_web::{
    dev::ServiceRequest,
    dev::ServiceResponse,
    http::{HeaderName, HeaderValue, Method},
    web::Data,
    Error, HttpRequest, HttpResponse,
};
use diesel::pg::PgConnection;
use futures::future::{ok, Ready};
use futures::Future;
use serde_json::json;
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        if should_skip_verify(&req) || verify_and_insert_auth_user(&mut req) {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(middleware::error::ErrorResponse::from(
                            constants::error_msg::UNAUTHRIZED,
                        ))
                        .into_body(),
                ))
            })
        }
    }
}

fn should_skip_verify(req: &ServiceRequest) -> bool {
    let method = req.method();
    if Method::OPTIONS == *method {
        return true;
    }
    for route in IGNORE_AUTH_ROUTES.iter() {
        if route.is_match_path_and_method(req.path(), req.method()) {
            return true;
        }
    }
    return false;
}

fn find_auth_user(conn: &PgConnection, user_id: Uuid) -> Result<User, AppError> {
    let user = User::find_by_id(&conn, user_id)?;
    Ok(user)
}

// const TOKEN_IDENTIFIER: &str = "Bearer";
const TOKEN_IDENTIFIER: &str = "Token";

fn verify_and_insert_auth_user(req: &mut ServiceRequest) -> bool {
    // TODO: Does it need?
    req.headers_mut().append(
        HeaderName::from_static("content-length"),
        HeaderValue::from_static("true"),
    );

    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        info!("Parsing authorization header...");
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with(TOKEN_IDENTIFIER) {
                info!("Parsing token...");
                let token = authen_str[6..authen_str.len()].trim();
                match token::decode(&token) {
                    Ok(token_data) => {
                        let claims = token_data.claims;
                        let user_id = claims.user_id;
                        if let Some(state) = req.app_data::<Data<AppState>>() {
                            let conn = state.get_conn();
                            match conn {
                                Ok(conn) => {
                                    match find_auth_user(&conn, user_id) {
                                        Ok(user) => {
                                            req.head().extensions_mut().insert(user);
                                            return true;
                                        }
                                        Err(_err) => {
                                            warn!("couldn't find auth user.");
                                            return false;
                                        }
                                    };
                                }
                                Err(_err) => {
                                    warn!("couldn't find auth user.");
                                    return false;
                                }
                            }
                        }
                        return false;
                    }
                    _ => {
                        error!("Invalid token");
                        return false;
                    }
                }
            }
        }
    };
    false
}

pub fn access_auth_user(req: &HttpRequest) -> Result<User, AppError> {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>();
    let auth_user = auth_user.map(|user| user.to_owned()); // TODO: avoid copy
    let auth_user = auth_user.ok_or(AppError::Unauthorized(
        json!({"error": "Unauthrized user. Need auth token on header."}),
    ))?;

    Ok(auth_user)
}

struct IgnoreAuthRoute {
    path: &'static str,
    method: Method,
}

impl IgnoreAuthRoute {
    fn is_match_path_and_method(&self, path: &str, method: &Method) -> bool {
        self.is_match_path(path) && self.is_match_method(method)
    }

    fn is_match_path(&self, path: &str) -> bool {
        let expect_path = self.path.split("/").collect::<Vec<_>>();
        let this_path = path.split("/").collect::<Vec<_>>();
        if expect_path.len() != this_path.len() {
            return false;
        };
        let path_set = expect_path.iter().zip(this_path.iter());
        for (expect_path, this_path) in path_set {
            if IgnoreAuthRoute::is_slug_path(*expect_path) {
                continue;
            }
            if expect_path != this_path {
                return false;
            }
        }
        return true;
    }

    fn is_match_method(&self, method: &Method) -> bool {
        self.method == method
    }

    fn is_slug_path(text: &str) -> bool {
        let first = text.chars().nth(0).unwrap_or(' ');
        let last = text.chars().last().unwrap_or(' ');
        first == '{' && last == '}'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::Method;
    #[test]
    fn is_match_path_and_method_test() {
        let route = IgnoreAuthRoute {
            path: "/api/healthcheck",
            method: Method::GET,
        };
        assert!(route.is_match_path_and_method("/api/healthcheck", &Method::GET));

        let route = IgnoreAuthRoute {
            path: "/api/{this-is-slug}/healthcheck",
            method: Method::POST,
        };
        assert!(route.is_match_path_and_method("/api/1234/healthcheck", &Method::POST));
    }
}

const IGNORE_AUTH_ROUTES: [IgnoreAuthRoute; 6] = [
    IgnoreAuthRoute {
        path: "/api/healthcheck",
        method: Method::GET,
    },
    IgnoreAuthRoute {
        path: "/api/tags",
        method: Method::GET,
    },
    IgnoreAuthRoute {
        path: "/api/users",
        method: Method::POST,
    },
    IgnoreAuthRoute {
        path: "/api/users/login",
        method: Method::POST,
    },
    IgnoreAuthRoute {
        path: "/api/articles",
        method: Method::GET,
    },
    IgnoreAuthRoute {
        path: "/api/articles/{article_title_slug}/comments",
        method: Method::GET,
    },
];
