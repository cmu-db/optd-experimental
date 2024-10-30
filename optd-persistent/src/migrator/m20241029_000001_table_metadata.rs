use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use super::namespace_schema::NamespaceSchema;

#[derive(Iden)]
pub enum TableMetadata {
    Table,
    Id,
    SchemaId,
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
                    .table(TableMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(TableMetadata::Id))
                    .col(integer(TableMetadata::SchemaId))
                    .col(string(TableMetadata::Name))
                    .col(timestamp(TableMetadata::CreatedTime))
                    .foreign_key(
                        ForeignKey::create()
                            .from(TableMetadata::Table, TableMetadata::SchemaId)
                            .to(NamespaceSchema::Table, NamespaceSchema::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TableMetadata::Table).to_owned())
            .await
    }
}
