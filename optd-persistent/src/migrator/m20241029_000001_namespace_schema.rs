use super::database_metadata::DatabaseMetadata;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(Iden)]
pub enum NamespaceSchema {
    Table,
    Id,
    DatabaseId,
    Name,
    CreatedTime,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NamespaceSchema::Table)
                    .if_not_exists()
                    .col(pk_auto(NamespaceSchema::Id))
                    .col(integer(NamespaceSchema::DatabaseId))
                    .col(string(NamespaceSchema::Name))
                    .col(timestamp(NamespaceSchema::CreatedTime))
                    .foreign_key(
                        ForeignKey::create()
                            .from(NamespaceSchema::Table, NamespaceSchema::DatabaseId)
                            .to(DatabaseMetadata::Table, DatabaseMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NamespaceSchema::Table).to_owned())
            .await
    }
}
