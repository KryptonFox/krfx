pub use sea_orm_migration::prelude::*;

mod m20250209_100325_create_users_table;
mod m20250209_100834_create_records_table;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250209_100325_create_users_table::Migration),
            Box::new(m20250209_100834_create_records_table::Migration),
        ]
    }
}
