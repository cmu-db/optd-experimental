use sea_orm_migration::prelude::*;

use super::table_metadata::TableMetadata;

#[derive(Iden)]
pub enum TableAttribute {
    Table,
    Id,
    TableId,
    Name,
    CompressionMethod,
    Type,
    BaseColNumber,
    IsNotNull,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TableAttribute::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TableAttribute::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(TableAttribute::TableId).integer())
                    .col(ColumnDef::new(TableAttribute::Name).string())
                    .col(ColumnDef::new(TableAttribute::CompressionMethod).char())
                    .col(ColumnDef::new(TableAttribute::Type).integer())
                    .col(ColumnDef::new(TableAttribute::BaseColNumber).integer())
                    .col(ColumnDef::new(TableAttribute::IsNotNull).boolean())
                    .foreign_key(
                        ForeignKey::create()
                            .from(TableAttribute::Table, TableAttribute::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TableAttribute::Table).to_owned())
            .await
    }
}
