/*
Table attribute_statistic {
  id integer PK
  name varchar
  table_id integer
  epoch_id integer
  created_time timestamp
  number_of_attributes integer
  statistic_type integer // Should we make another table to explain the type mapping?
  statistic_value float
  Ref: attribute_statistic.epoch_id > event.epoch_id
  Ref: attribute_statistic.table_id > table_metadata.id
} */

use crate::migrator::{
    catalog::m20241029_000001_table_metadata::TableMetadata, cost_model::event::Event,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum AttributeStatistic {
    Table,
    Id,
    Name,
    TableId,
    EpochId,
    CreatedTime,
    NumberOfAttributes,
    StatisticType,
    StatisticValue,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AttributeStatistic::Table)
                    .if_not_exists()
                    .col(pk_auto(AttributeStatistic::Id))
                    .col(string(AttributeStatistic::Name))
                    .col(integer(AttributeStatistic::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AttributeStatistic::Table, AttributeStatistic::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(AttributeStatistic::EpochId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AttributeStatistic::Table, AttributeStatistic::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(AttributeStatistic::CreatedTime))
                    .col(integer(AttributeStatistic::NumberOfAttributes))
                    .col(integer(AttributeStatistic::StatisticType))
                    .col(float(AttributeStatistic::StatisticValue))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AttributeStatistic::Table).to_owned())
            .await
    }
}
