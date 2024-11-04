use crate::migrator::cost_model::database_metadata::DatabaseMetadata;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(Iden)]
pub enum NamespaceMetadata {
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
                    .table(NamespaceMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(NamespaceMetadata::Id))
                    .col(integer(NamespaceMetadata::DatabaseId))
                    .col(string(NamespaceMetadata::Name))
                    .col(timestamp(NamespaceMetadata::CreatedTime))
                    .foreign_key(
                        ForeignKey::create()
                            .from(NamespaceMetadata::Table, NamespaceMetadata::DatabaseId)
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
            .drop_table(Table::drop().table(NamespaceMetadata::Table).to_owned())
            .await
    }
}
