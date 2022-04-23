use crate::app::user::model::User;
use crate::constants;
use crate::error::AppError;
use crate::middleware::state::AppState;
use crate::utils::token;
use actix_web::HttpMessage;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    web::Data,
    Error, HttpRequest, HttpResponse,
};
use diesel::pg::PgConnection;
use futures::future::{ok, Ready};
use futures::Future;
use serde_json::json;
use std::pin::Pin;
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
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

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;

    #[allow(clippy::type_complexity)] // TODO: want to remove allowness to skip and refactor somehow
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let is_verified = if should_skip_verification(&req) {
            true
        } else {
            verify_and_insert_auth_user(&mut req)
        };
        if is_verified {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?.map_into_left_body();
                Ok(res)
            })
        } else {
            Box::pin(async move {
                let (req, _res) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                let srv = ServiceResponse::new(req, res);
                Ok(srv)
            })
        }
    }
}

fn should_skip_verification(req: &ServiceRequest) -> bool {
    let method = req.method();
    if Method::OPTIONS == *method {
        return true;
    }

    IGNORE_AUTH_ROUTES
        .iter()
        .any(|route| route.is_match_path_and_method(req.path(), req.method()))
}

fn find_auth_user(conn: &PgConnection, user_id: Uuid) -> Result<User, AppError> {
    let user = User::find(conn, user_id)?;
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

    let auth_header = match req.headers().get(constants::AUTHORIZATION) {
        Some(auth_header) => auth_header,
        None => return false,
    };

    info!("Parsing authorization header...");

    let auth_str = match auth_header.to_str() {
        Ok(auth_str) => auth_str,
        _ => return false,
    };

    if !&auth_str.starts_with(TOKEN_IDENTIFIER) {
        return false;
    };

    info!("Parsing token...");

    let token = auth_str[6..auth_str.len()].trim();

    let token_data = match token::decode(token) {
        Ok(token_data) => token_data,
        Err(_err) => {
            error!("Invalid token");
            return false;
        }
    };

    let claims = token_data.claims;
    let user_id = claims.user_id;

    let state = match req.app_data::<Data<AppState>>() {
        Some(state) => state,
        None => return false,
    };

    let conn = state.get_conn();

    let conn = match conn {
        Ok(conn) => conn,
        Err(_err) => {
            warn!("couldn't find auth user.");
            return false;
        }
    };

    let user = match find_auth_user(&conn, user_id) {
        Ok(user) => user,
        Err(_err) => {
            warn!("couldn't find auth user.");
            return false;
        }
    };

    req.extensions_mut().insert(user);
    true
}

pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
    let user = req.extensions();
    let user = user.get::<User>();
    let user = user.map(|user| user.to_owned()); // TODO: avoid copy
    let user = user.ok_or_else(|| {
        AppError::Unauthorized(json!({"error": "Unauthrized user. Need auth token on header."}))
    })?;

    Ok(user)
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
        let expect_path = self.path.split('/').collect::<Vec<_>>();
        let this_path = path.split('/').collect::<Vec<_>>();
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
        true
    }

    fn is_match_method(&self, method: &Method) -> bool {
        self.method == method
    }

    fn is_slug_path(text: &str) -> bool {
        let first = text.chars().next().unwrap_or(' ');
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
