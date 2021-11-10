pub const IGNORE_AUTH_ROUTES: [&str; 4] = [
    "/api/healthcheck",
    "/api/tags",
    "/api/users",
    "/api/users/login",
];
// pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
// pub const EMPTY: &str = "";
pub const AUTHORIZATION: &str = "Authorization";

pub const BIND: &str = "0.0.0.0:8080";

pub mod env_key {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const FRONTEND_ORIGIN: &str = "FRONTEND_ORIGIN";
}

pub mod error_msg {
    pub const UNAUTHRIZED: &str = "Unauthrized user, please signin.";
}
