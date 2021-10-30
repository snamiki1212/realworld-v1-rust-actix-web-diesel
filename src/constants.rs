pub const IGNORE_AUTH_ROUTES: [&str; 3] = ["/api/tags", "/api/users", "/api/users/login"];
// pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
// pub const EMPTY: &str = "";
pub const AUTHORIZATION: &str = "Authorization";

pub mod env_key {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const FRONTEND_ORIGIN: &str = "FRONTEND_ORIGIN";
}

pub mod error_msg {
    pub const UNAUTHRIZED: &str = "Unauthrized user, please signin.";
}
