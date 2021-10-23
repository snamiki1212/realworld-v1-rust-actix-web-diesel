pub mod response {
    use crate::app::user::model::User;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct UserResponse {
        pub user: AuthUser,
    }
    impl UserResponse {
        pub fn from(user: User, token: String) -> Self {
            // REF: https://gothinkster.github.io/realworld/docs/specs/backend-specs/api-response-format/#users-for-authentication
            Self {
                user: AuthUser {
                    email: user.email,
                    token: token,
                    username: user.username,
                    // bio: user.bio,
                    // image: user.image,
                },
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AuthUser {
        pub email: String,
        pub token: String,
        pub username: String,
    }
}

pub mod request {
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
}
