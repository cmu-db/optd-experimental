use crate::migrator::catalog::namespace_metadata::NamespaceMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum TableMetadata {
    Table,
    Id,
    Name,
    NamespaceId,
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
                    .table(TableMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(TableMetadata::Id))
                    .col(string(TableMetadata::Name))
                    .col(integer(TableMetadata::NamespaceId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(TableMetadata::Table, TableMetadata::NamespaceId)
                            .to(NamespaceMetadata::Table, NamespaceMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(TableMetadata::CreationTime))
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
