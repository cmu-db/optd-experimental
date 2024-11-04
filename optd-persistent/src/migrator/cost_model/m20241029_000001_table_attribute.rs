use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::migrator::cost_model::table_metadata::TableMetadata;

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
                    .col(pk_auto(TableAttribute::Id))
                    .col(integer(TableAttribute::TableId))
                    .col(string(TableAttribute::Name))
                    .col(char(TableAttribute::CompressionMethod))
                    .col(integer(TableAttribute::Type))
                    .col(integer(TableAttribute::BaseColNumber))
                    .col(boolean(TableAttribute::IsNotNull))
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
