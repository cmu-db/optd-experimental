/*
Table attribute_stat {
  id integer PK
  number_of_attributes integer // do we need it?
  data json // stores the related table id and attribute ids
  epoch_id integer
  name varchar
  created_time timestamp
  stats_type integer // Should we make another table to explain the type mapping?
  stats_value integer // Can we represent every stats value into integer?
  Ref: attribute_stats.epoch_id > event.epoch_id
} */

use crate::migrator::cost_model::event::Event;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum ColumnStatistic {
    Table,
    Id,
    Name,
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
                    .table(ColumnStatistic::Table)
                    .if_not_exists()
                    .col(pk_auto(ColumnStatistic::Id))
                    .col(string(ColumnStatistic::Name))
                    .col(integer(ColumnStatistic::EpochId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ColumnStatistic::Table, ColumnStatistic::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(ColumnStatistic::CreatedTime))
                    .col(integer(ColumnStatistic::NumberOfAttributes))
                    .col(integer(ColumnStatistic::StatisticType))
                    .col(integer(ColumnStatistic::StatisticValue))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ColumnStatistic::Table).to_owned())
            .await
    }
}
