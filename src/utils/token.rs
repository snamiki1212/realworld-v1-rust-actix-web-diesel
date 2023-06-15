use crate::constants::env_key;
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

fn get_secret_key() -> String {
    env::var(env_key::SECRET_KEY).expect("SECRET_KEY must be set")
}

pub fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    let binding = get_secret_key();
    let secret_key = binding.as_bytes();
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key),
        &Validation::default(),
    )
}

pub fn generate(user_id: Uuid, now: i64) -> Result<String, Error> {
    let claims = Claims::new(user_id, now);
    let binding = get_secret_key();
    let secret_key = binding.as_bytes();
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // aud: String, // Optional. Audience
    exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    // sub: String, // Optional. Subject (whom token refers to)
    // ---
    pub user_id: Uuid,
}

impl Claims {
    pub fn new(user_id: Uuid, now: i64) -> Self {
        Claims {
            iat: now,
            exp: now + ONE_DAY,
            user_id,
        }
    }
}
