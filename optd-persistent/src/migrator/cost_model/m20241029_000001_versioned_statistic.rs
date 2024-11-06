use crate::migrator::cost_model::{event::Event, statistic::Statistic};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum VersionedStatistic {
    Table,
    Id,
    EpochId,
    StatisticId,
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
                    .table(VersionedStatistic::Table)
                    .if_not_exists()
                    .col(pk_auto(VersionedStatistic::Id))
                    .col(integer(VersionedStatistic::EpochId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(VersionedStatistic::Table, VersionedStatistic::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(VersionedStatistic::StatisticId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(VersionedStatistic::Table, VersionedStatistic::StatisticId)
                            .to(Statistic::Table, Statistic::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(json(VersionedStatistic::StatisticValue))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VersionedStatistic::Table).to_owned())
            .await
    }
}
