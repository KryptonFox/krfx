use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(big_integer(User::Id).primary_key())
                    .col(string(User::Username).unique_key())
                    .col(string(User::DisplayName))
                    .col(date_time(User::CreatedAt))
                    .col(string(User::Password))
                    .col(boolean(User::IsAdmin).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    DisplayName,
    CreatedAt,
    Password,
    IsAdmin,
}
