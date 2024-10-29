use sea_orm_migration::{prelude::*, schema::*};

use super::m20241029_000001_namespace_metadata::NamespaceMetadata;

#[derive(DeriveIden)]
pub enum TableMetadata {
    Table,
    Id,
    CreateTime,
    Name,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_table_metadata"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TableMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(TableMetadata::Id))
                    .col(timestamp(TableMetadata::CreateTime).default("CURRENT_TIMESTAMP"))
                    .col(string(TableMetadata::Name).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-namespace_id")
                            .from(TableMetadata::Table, TableMetadata::Id)
                            .to(NamespaceMetadata::Table, NamespaceMetadata::Id)
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
