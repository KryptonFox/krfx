use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::types::JwtPayload;

fn timestamp() -> usize {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize
}

pub fn create_token(user_id: i64, secret: &String) -> jsonwebtoken::errors::Result<String> {
    let claims = JwtPayload {
        user_id,
        iat: timestamp(),
        iss: "krfx-web-rust".to_string(),
        exp: timestamp() + 7 * 24 * 60 * 60 * 1000,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}