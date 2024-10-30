use super::database_metadata::DatabaseMetadata;
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(NamespaceSchema::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(NamespaceSchema::DatabaseId).integer())
                    .col(ColumnDef::new(NamespaceSchema::Name).string())
                    .col(ColumnDef::new(NamespaceSchema::CreatedTime).timestamp())
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
