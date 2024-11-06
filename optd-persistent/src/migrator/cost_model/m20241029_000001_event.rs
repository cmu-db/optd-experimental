//! Every time we insert/update statistics, we need to insert a new
//! row into this table to record the event.

use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Event {
    Table,
    EpochId,
    Timestamp,
    SourceVariant,
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
                    .table(Event::Table)
                    .if_not_exists()
                    .col(pk_auto(Event::EpochId))
                    .col(timestamp(Event::Timestamp))
                    .col(string(Event::SourceVariant))
                    .col(json(Event::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}
