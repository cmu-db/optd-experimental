/*
Table attribute_stats_junction {
  attr_id integer [ref: > table_attribute.id]
  stats_id integer [ref: > attribute_stats.id]
}
 */

use crate::migrator::catalog::column::Column;
use crate::migrator::cost_model::column_statistic::ColumnStatistic;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum ColumnStatisticToColumnJunction {
    Table,
    ColumnStatisticId,
    ColumnId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ColumnStatisticToColumnJunction::Table)
                    .if_not_exists()
                    .col(integer(ColumnStatisticToColumnJunction::ColumnStatisticId))
                    .col(integer(ColumnStatisticToColumnJunction::ColumnId))
                    .primary_key(
                        Index::create()
                            .col(ColumnStatisticToColumnJunction::ColumnStatisticId)
                            .col(ColumnStatisticToColumnJunction::ColumnId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ColumnStatisticToColumnJunction::Table,
                                ColumnStatisticToColumnJunction::ColumnStatisticId,
                            )
                            .to(ColumnStatistic::Table, ColumnStatistic::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ColumnStatisticToColumnJunction::Table,
                                ColumnStatisticToColumnJunction::ColumnId,
                            )
                            .to(Column::Table, Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ColumnStatisticToColumnJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
