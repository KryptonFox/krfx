use crate::auth::create_token::create_token;
use crate::types::AppState;
use crate::utlis::string_hash_sha256;
use actix_web::cookie::Cookie;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use entity::prelude::User;
use entity::user;
use k_snowflake::create_snowflake;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
struct SignupPayload {
    username: String,
    password: String,
}

#[post("/signup")]
pub async fn signup(
    state: web::Data<AppState>,
    payload: web::Json<SignupPayload>,
) -> impl Responder {
    // check if username exists in db
    if let Ok(Some(_)) = User::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&state.conn)
        .await
    {
        HttpResponse::BadRequest().body("Username already exists");
    }
    // hash password
    let password_hash = string_hash_sha256(&payload.password, &state.env.salt);

    // write user in db
    let user = user::ActiveModel {
        id: Set(create_snowflake().to_decimal()),
        username: Set(payload.username.to_lowercase()),
        display_name: Set(payload.username.clone()),
        created_at: Set(Utc::now().naive_utc()),
        password: Set(password_hash),
    };
    User::insert(user.clone()).exec(&state.conn).await.unwrap();

    // create token
    let Ok(token) = create_token(user.id.unwrap(), &state.env.secret) else {
        return HttpResponse::InternalServerError().body("Could not create token");
    };

    // create cookie
    let cookie = Cookie::build("token", token)
        .secure(true)
        .http_only(true)
        .finish();

    // response with cookies
    HttpResponse::Ok()
        .cookie(cookie)
        .body("Successfully signed up")
}
