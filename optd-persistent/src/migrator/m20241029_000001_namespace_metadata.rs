use sea_orm_migration::{prelude::*, schema::*};

use super::m20241029_000001_database_metadata::DatabaseMetadata;

#[derive(DeriveIden)]
pub enum NamespaceMetadata {
    Table,
    Id,
    CreateTime,
    Name,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_namespace_metadata"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NamespaceMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(NamespaceMetadata::Id))
                    .col(timestamp(NamespaceMetadata::CreateTime).default("CURRENT_TIMESTAMP"))
                    .col(string(NamespaceMetadata::Name).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-database_id")
                            .from(NamespaceMetadata::Table, NamespaceMetadata::Id)
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
