use sea_orm::DatabaseConnection;
use migration::{Migrator, MigratorTrait};
use crate::config::ConfigEnvironment;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub env: ConfigEnvironment,
}

impl AppState {
    pub async fn new() -> Self {
        // load env variables
        let env = ConfigEnvironment::from_env();
        
        // connect db
        let conn = sea_orm::Database::connect(&env.database_url).await.unwrap();
        
        // apply all migrations
        Migrator::up(&conn, None).await.unwrap();

        Self {
            conn,
            env,
        }
    }
}