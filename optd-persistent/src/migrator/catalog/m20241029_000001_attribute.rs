use crate::migrator::catalog::table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Attribute {
    Table,
    Id,
    TableId,
    Name,
    CompressionMethod,
    Type,
    BaseAttributeNumber,
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
                    .table(Attribute::Table)
                    .if_not_exists()
                    .col(pk_auto(Attribute::Id))
                    .col(integer(Attribute::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attribute::Table, Attribute::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Attribute::Name))
                    .col(char(Attribute::CompressionMethod))
                    .col(integer(Attribute::Type))
                    .col(integer(Attribute::BaseAttributeNumber))
                    .col(boolean(Attribute::IsNotNull))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attribute::Table).to_owned())
            .await
    }
}
