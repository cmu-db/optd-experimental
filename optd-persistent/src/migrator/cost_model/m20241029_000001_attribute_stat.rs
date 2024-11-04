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
pub enum AttributeStat {
    Table,
    Id,
    NumberOfAttributes,
    EpochId,
    Name,
    CreatedTime,
    StatsType,
    StatsValue,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AttributeStat::Table)
                    .if_not_exists()
                    .col(pk_auto(AttributeStat::Id))
                    .col(integer(AttributeStat::NumberOfAttributes))
                    .col(integer(AttributeStat::EpochId))
                    .col(string(AttributeStat::Name))
                    .col(timestamp(AttributeStat::CreatedTime))
                    .col(integer(AttributeStat::StatsType))
                    .col(integer(AttributeStat::StatsValue))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AttributeStat::Table, AttributeStat::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AttributeStat::Table).to_owned())
            .await
    }
}
