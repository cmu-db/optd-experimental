//! This table stores the statistic infos. One sole statistic only has one row in this table.
//!
//! If we want to insert a new statistic, we should first insert one row into this table, then add a new
//! event, and finally insert the statistic value into the versioned_statistic table.
//! If we want to update a statistic, we should first find the real statistic id from this table, then
//! add a new event, and finally insert the statistic value into the versioned_statistic table.

use crate::migrator::catalog::m20241029_000001_table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Statistic {
    Table,
    Id,
    Name,
    // null if not a table statistic.
    TableId,
    CreatedTime,
    // 0 if a table statistic.
    NumberOfAttributes,
    // TODO(lanlou): Should we make another table to explain the type mapping?
    #[allow(clippy::enum_variant_names)]
    StatisticType,
    // Store the sorted attribute ids of this statistic, to support quick lookup (OR we can use junction table to look up)
    // For example, if we want to store the statistic of attributes [1, 2, 3], we can store it as "1,2,3".
    // During lookup, we should first sort the attribute ids, and then look up.
    // OR we can use statistic_to_attribute_junction table to look up.
    Description,
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
                    .col(string(Statistic::Description))
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
