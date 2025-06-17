use crate::services::AuthService;
use actix_web::{error, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login_handler(
    auth_service: web::Data<AuthService>,
    payload: web::Json<LoginPayload>,
) -> error::Result<HttpResponse> {
    // login by auth service and obtain cookie from it
    let cookie = auth_service.login(&payload.username, &payload.password).await?;

    // response with cookies
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body("Successfully logged in"))
}
