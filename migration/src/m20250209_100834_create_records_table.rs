use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Record::Table)
                    .if_not_exists()
                    .col(big_integer(Record::Id).primary_key())
                    .col(big_integer_null(Record::OwnerId))
                    .col(string_uniq(Record::Name))
                    .col(string_uniq(Record::VisibleName))
                    .col(date_time(Record::CreatedAt))
                    .col(string(Record::Url))
                    .col(boolean(Record::IsFile).default(false))
                    .col(string_null(Record::Hash))
                    .col(string_null(Record::MimeType))
                    .col(boolean(Record::IsTemp).default(false))
                    .col(date_time_null(Record::ExpiresAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Record::Table, Record::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Record::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Record {
    Table,
    Id,
    OwnerId,
    Name,
    VisibleName,
    CreatedAt,
    Url,
    IsFile,
    // file specific
    Hash,
    MimeType,
    // temp records
    IsTemp,
    ExpiresAt,
}
