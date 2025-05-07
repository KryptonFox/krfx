use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub env: Environment,
}

impl AppState {
    pub async fn new(env: &Environment) -> Self {
        // connect db
        let conn = sea_orm::Database::connect(&env.database_url).await.unwrap();
        Migrator::up(&conn, None).await.unwrap();

        Self {
            conn,
            env: env.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Environment {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database_url: String,
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

#[derive(Clone, Debug)]
pub enum UserType {
    User(i64),
    Admin(i64),
    None,
}

impl FromRequest for UserType {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match req.extensions().get::<UserType>() {
                Some(user_type) => Ok(user_type.to_owned()),
                None => Err(actix_web::error::ErrorInternalServerError(
                    "Cannot parse user type",
                )),
            }
        })
    }
}
