/*
Table statistic {
  id integer PK
  name varchar
  table_id integer // 0 if not a table statistic
  epoch_id integer
  created_time timestamp
  number_of_attributes integer // 0 if a table constraint
  statistic_type integer // Should we make another table to explain the type mapping?
  statistic_value float
  Ref: statistic.epoch_id > event.epoch_id
  Ref: statistic.table_id > table_metadata.id
}
*/

use crate::migrator::{
    catalog::m20241029_000001_table_metadata::TableMetadata, cost_model::event::Event,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Statistic {
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
                    .col(integer(Statistic::EpochId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Statistic::Table, Statistic::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(Statistic::CreatedTime))
                    .col(integer(Statistic::NumberOfAttributes))
                    .col(integer(Statistic::StatisticType))
                    .col(float(Statistic::StatisticValue))
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
