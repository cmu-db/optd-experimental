//! An entity representing a the best physical plan (or "winner") of a Cascades group.
//!
//! In the Cascades framework, query optimization is done through dynamic programming that is based
//! on the assumption that the cost model satisfies the _principle of optimality_. Quoted from the
//! Microsoft article _Extensible query optimizers in practice_:
//!
//! > ... in the search space of linear sequence of joins, the optimal plan for a join of n
//! > relations can be found by extending the optimal plan of a sub-expression of n - 1 joins with
//! > an additional join.
//!
//! By storing the best sub-plans / [`physical_expression`]s of smaller Cascades groups, we can
//! build up an optimal query plan.
//!
//! This entity represents the best plan sub-tree for a specific group. However, we store multiple
//! winners over different epochs, as changes to the database may require us to re-evaluate what the
//! optimal sub-plan is.
//!
//! # Columns
//!
//! Other than the primary key, all of the columns in this relation are foreign keys to other
//! tables.
//!
//! A group winner is defined by the [`cascades_group`] it belongs to (`group_id`), the unique ID of
//! the [`physical_expression`] (`physical_expression_id`), the ID of the cost record in the
//! [`plan_cost`] table (`cost_id`), and the monotonically-increasing epoch ID in the [`event`]
//! table (`epoch_id`).
//!
//! [`cascades_group`]: super::cascades_group
//! [`physical_expression`]: super::physical_expression
//! [`plan_cost`]: super::super::cost_model::plan_cost
//! [`event`]: super::super::cost_model::event

use crate::migrator::cost_model::{event::Event, plan_cost::PlanCost};
use crate::migrator::memo::{
    cascades_group::CascadesGroup, physical_expression::PhysicalExpression,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum GroupWinner {
    Table,
    Id,
    GroupId,
    PhysicalExpressionId,
    CostId,
    EpochId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GroupWinner::Table)
                    .if_not_exists()
                    .col(pk_auto(GroupWinner::Id))
                    .col(integer(GroupWinner::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(GroupWinner::PhysicalExpressionId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::PhysicalExpressionId)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(GroupWinner::CostId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::CostId)
                            .to(PlanCost::Table, PlanCost::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(GroupWinner::EpochId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::EpochId)
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
            .drop_table(Table::drop().table(GroupWinner::Table).to_owned())
            .await
    }
}
