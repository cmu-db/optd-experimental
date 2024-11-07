use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum DatabaseMetadata {
    Table,
    Id,
    Name,
    CreationTime,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DatabaseMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(DatabaseMetadata::Id))
                    .col(string(DatabaseMetadata::Name))
                    .col(timestamp(DatabaseMetadata::CreationTime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DatabaseMetadata::Table).to_owned())
            .await
    }
}
