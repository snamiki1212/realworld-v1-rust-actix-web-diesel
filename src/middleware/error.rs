use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Inner,
}

impl ErrorResponse {
    pub fn from(msg: &str) -> Self {
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
