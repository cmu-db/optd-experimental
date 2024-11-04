use crate::migrator::catalog::table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Column {
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
                    .table(Column::Table)
                    .if_not_exists()
                    .col(pk_auto(Column::Id))
                    .col(integer(Column::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Column::Table, Column::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Column::Name))
                    .col(char(Column::CompressionMethod))
                    .col(integer(Column::Type))
                    .col(integer(Column::BaseColNumber))
                    .col(boolean(Column::IsNotNull))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Column::Table).to_owned())
            .await
    }
}
