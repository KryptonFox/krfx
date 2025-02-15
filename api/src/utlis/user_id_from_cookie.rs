use crate::types::{AppState, JwtPayload};
use actix_web::{web, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};

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
