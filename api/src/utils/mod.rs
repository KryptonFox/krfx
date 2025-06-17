use crate::state::AppState;
use crate::types::JwtPayload;
use actix_web::{web, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sha2::{Digest, Sha256};

pub fn get_user_id_from_cookie(req: &HttpRequest, state: &web::Data<AppState>) -> Option<i64> {
    req.cookie("token")
        .and_then(|cookie| {
            decode::<JwtPayload>(
                &cookie.value(),
                &DecodingKey::from_secret(state.env.secret.as_bytes()),
                &Validation::default(),
            )
            .ok()
        })
        .map(|token| token.claims.user_id)
}

/// Return string with hex value of SHA256 hash of value + salt
pub fn string_hash_sha256(value: impl Into<String>, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update((value.into() + salt).as_bytes());
    format!("{:x}", hasher.finalize())
}
