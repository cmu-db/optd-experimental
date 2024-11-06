use crate::migrator::catalog::database_metadata::DatabaseMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum NamespaceMetadata {
    Table,
    Id,
    Name,
    DatabaseId,
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
                    .table(NamespaceMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(NamespaceMetadata::Id))
                    .col(string(NamespaceMetadata::Name))
                    .col(integer(NamespaceMetadata::DatabaseId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(NamespaceMetadata::Table, NamespaceMetadata::DatabaseId)
                            .to(DatabaseMetadata::Table, DatabaseMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(NamespaceMetadata::CreationTime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NamespaceMetadata::Table).to_owned())
            .await
    }
}
