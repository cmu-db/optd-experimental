use crate::migrator::catalog::m20241029_000001_table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Statistic {
    Table,
    Id,
    Name,
    TableId,
    CreatedTime,
    NumberOfAttributes,
    StatisticType,
    Data,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Statistic::Table)
                    .if_not_exists()
                    .col(pk_auto(Statistic::Id))
                    .col(string(Statistic::Name))
                    .col(integer(Statistic::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Statistic::Table, Statistic::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(Statistic::CreatedTime))
                    .col(integer(Statistic::NumberOfAttributes))
                    .col(integer(Statistic::StatisticType))
                    .col(string(Statistic::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Statistic::Table).to_owned())
            .await
    }
}
