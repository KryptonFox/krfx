use crate::types::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use entity::prelude::User;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use crate::auth::create_token::create_token;

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, payload: web::Json<LoginPayload>) -> impl Responder {
    // hash password
    let mut hasher = Sha256::new();
    hasher.update(&payload.password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());

    // find user
    let Ok(Some(user)) = User::find()
        .filter(user::Column::Username.eq(&payload.username))
        .filter(user::Column::Password.eq(&password_hash))
        .one(&state.conn)
        .await
    else {
        return HttpResponse::BadRequest().body("Password or username are incorrect")
    };

    // create token
    let Ok(token) = create_token(user.id, &state.env.secret) else {
        return HttpResponse::InternalServerError().body("Could not create token")
    };

    // create cookie
    let cookie = Cookie::build("token", token)
        .secure(true)
        .http_only(true)
        .finish();

    // response with cookies
    HttpResponse::Ok().cookie(cookie).body("Successfully signed up")
}
