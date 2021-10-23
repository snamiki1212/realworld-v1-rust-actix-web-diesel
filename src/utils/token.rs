use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

static KEY: [u8; 16] = *include_bytes!("../../secret.key"); // TODO:
static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn generate(now: i64) -> Result<String, Error> {
    let claims = Claims::new(now);
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(&KEY));
    token
}

pub fn verify(token: &str) -> bool {
    decode(token).is_ok()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // aud: String,         // Optional. Audience
    exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issued at (as UTC timestamp)
              // iss: String,         // Optional. Issuer
              // nbf: usize,          // Optional. Not Before (as UTC timestamp)
              // sub: String,         // Optional. Subject (whom token refers to)
}
impl Claims {
    pub fn new(now: i64) -> Self {
        Claims {
            iat: now,
            exp: now + ONE_DAY,
        }
    }
}
