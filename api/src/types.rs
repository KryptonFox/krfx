use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

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
