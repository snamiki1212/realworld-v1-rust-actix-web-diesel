use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Signup {
    // SPEC: https://gothinkster.github.io/realworld/docs/specs/backend-specs/endpoints#registration
    pub user: SignupUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Signin {
    // SPEC: https://gothinkster.github.io/realworld/docs/specs/backend-specs/endpoints#authentication
    pub user: SigninUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Update {
    // SPEC: https://gothinkster.github.io/realworld/docs/specs/backend-specs/endpoints#authentication
    pub user: UpdateUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}
