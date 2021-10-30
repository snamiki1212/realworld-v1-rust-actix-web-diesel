use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Inner,
}

impl std::convert::From<&str> for ErrorResponse {
    fn from(msg: &str) -> Self {
        Self {
            errors: Inner {
                body: vec![msg.to_owned()],
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Inner {
    body: Vec<String>,
}
