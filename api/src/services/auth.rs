use crate::state::AppState;
use crate::types::JwtPayload;
use crate::utils::string_hash_sha256;
use actix_web::cookie::Cookie;
use actix_web::error;
use chrono::Utc;
use entity::prelude::User;
use entity::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use k_snowflake::create_snowflake;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthService {
    app_state: Arc<AppState>,
}

impl AuthService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn login(&self, username: &String, password: &String) -> error::Result<Cookie<'_>> {
        // hash password
        let password_hash = string_hash_sha256(password.as_str(), &self.app_state.env.salt);

        // find user
        let user = User::find()
            .filter(user::Column::Username.eq(&username.to_lowercase()))
            .filter(user::Column::Password.eq(&password_hash))
            .one(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)?
            .ok_or(error::ErrorBadRequest("Password or username are incorrect"))?;

        let token = self
            .create_auth_token(user.id)
            .map_err(error::ErrorInternalServerError)?;

        Ok(Self::create_token_cookie(token))
    }

    pub async fn signup(&self, username: &String, password: &String) -> error::Result<Cookie<'_>> {
        // check if username exists in db
        if User::find()
            .filter(user::Column::Username.eq(username.to_lowercase().as_str()))
            .one(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)?
            .is_some()
        {
            return Err(error::ErrorBadRequest("Username already exists"));
        }
        // hash password
        let password_hash = string_hash_sha256(password.as_str(), &self.app_state.env.salt);

        // write user in db
        let user = user::ActiveModel {
            id: Set(create_snowflake().to_decimal()),
            username: Set(username.to_lowercase()),
            display_name: Set(username.clone()),
            created_at: Set(Utc::now().naive_utc()),
            password: Set(password_hash),
            ..Default::default()
        };
        let user = user.insert(&self.app_state.conn).await.unwrap();

        let token = self
            .create_auth_token(user.id)
            .map_err(error::ErrorInternalServerError)?;

        Ok(Self::create_token_cookie(token))
    }

    fn create_auth_token(&self, user_id: i64) -> jsonwebtoken::errors::Result<String> {
        let secret = &self.app_state.env.secret;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        let claims = JwtPayload {
            user_id,
            iat: timestamp,
            iss: "krfx-web-rust".to_string(),
            exp: timestamp + (7 * 24 * 60 * 60 * 1000),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    fn create_token_cookie<'c>(token: String) -> Cookie<'c> {
        Cookie::build("token", token)
            .secure(true)
            .http_only(true)
            .path("/api")
            .finish()
    }
}
