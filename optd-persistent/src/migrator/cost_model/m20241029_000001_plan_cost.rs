//! When a statistic is updated, then all the related costs should be invalidated. (IsValid is set to false)
//! This design (using IsValid flag) is based on the assumption that update_stats will not be called very frequently.
//! It favors the compute_cost performance over the update_stats performance.

use crate::migrator::cost_model::event::Event;
use crate::migrator::memo::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum PlanCost {
    Table,
    Id,
    PhysicalExpressionId,
    EpochId,
    // It is json type, including computation cost, I/O cost, etc.
    Cost,
    // Raw estimated output row count of this expression
    EstimatedStatistic,
    // Whether the cost is valid or not. If the latest cost for an expr is invalid, then we need to recompute the cost.
    // We need to invalidate the cost when the related stats are updated.
    IsValid,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlanCost::Table)
                    .if_not_exists()
                    .col(pk_auto(PlanCost::Id))
                    .col(integer(PlanCost::PhysicalExpressionId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlanCost::Table, PlanCost::PhysicalExpressionId)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PlanCost::EpochId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlanCost::Table, PlanCost::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(json(PlanCost::Cost))
                    .col(integer(PlanCost::EstimatedStatistic))
                    .col(boolean(PlanCost::IsValid))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlanCost::Table).to_owned())
            .await
    }
}