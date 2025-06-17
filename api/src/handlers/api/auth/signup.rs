use actix_web::{error, post, web, HttpResponse};
use serde::Deserialize;
use crate::services::AuthService;

#[derive(Deserialize)]
struct SignupPayload {
    username: String,
    password: String,
}

#[post("/signup")]
pub async fn signup_handler(
    auth_service: web::Data<AuthService>,
    payload: web::Json<SignupPayload>,
) -> error::Result<HttpResponse> {
    // signup and auth by auth service and obtain cookie from it
    let cookie = auth_service.signup(&payload.username, &payload.password).await?;

    // response with cookies
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body("Successfully signed up"))
}