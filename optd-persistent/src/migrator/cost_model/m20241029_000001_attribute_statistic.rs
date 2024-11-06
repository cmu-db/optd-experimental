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
pub enum AttributeStatistic {
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
                    .table(AttributeStatistic::Table)
                    .if_not_exists()
                    .col(pk_auto(AttributeStatistic::Id))
                    .col(string(AttributeStatistic::Name))
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
                    .col(integer(AttributeStatistic::StatisticValue))
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
