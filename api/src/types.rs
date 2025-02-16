use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub env: Environment,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Environment {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub instance: u16,
    pub secret: String,
    pub salt: String,
    pub bucket_name: String,
    pub cdn_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user_id: i64,
    pub iat: usize,
    pub iss: String,
    pub exp: usize,
}
