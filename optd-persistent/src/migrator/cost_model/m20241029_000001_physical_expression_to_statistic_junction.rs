use crate::migrator::cost_model::statistic::Statistic;
use crate::migrator::memo::physical_expression::PhysicalExpression;

use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum PhysicalExpressionToStatisticJunction {
    Table,
    PhysicalExpressionId,
    StatisticId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhysicalExpressionToStatisticJunction::Table)
                    .if_not_exists()
                    .col(integer(
                        PhysicalExpressionToStatisticJunction::PhysicalExpressionId,
                    ))
                    .col(integer(PhysicalExpressionToStatisticJunction::StatisticId))
                    .primary_key(
                        Index::create()
                            .col(PhysicalExpressionToStatisticJunction::PhysicalExpressionId)
                            .col(PhysicalExpressionToStatisticJunction::StatisticId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PhysicalExpressionToStatisticJunction::Table,
                                PhysicalExpressionToStatisticJunction::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PhysicalExpressionToStatisticJunction::Table,
                                PhysicalExpressionToStatisticJunction::StatisticId,
                            )
                            .to(Statistic::Table, Statistic::Id)
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
                    .table(PhysicalExpressionToStatisticJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
