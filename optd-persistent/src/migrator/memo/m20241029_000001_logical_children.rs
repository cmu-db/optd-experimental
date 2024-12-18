//! An entity representing the [`cascades_group`] children of every [`logical_expression`].
//!
//! Formally, this entity is a junction which allows us to represent a many-to-many relationship
//! between [`logical_expression`] and [`cascades_group`]. Expressions can have any number of child
//! groups, and every group can be a child of many different expressions, hence the many-to-many
//! relationship.
//!
//! See [`cascades_group`] for more details.
//!
//! [`cascades_group`]: super::cascades_group
//! [`logical_expression`]: super::logical_expression

use crate::migrator::memo::{cascades_group::CascadesGroup, logical_expression::LogicalExpression};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalChildren {
    Table,
    LogicalExpressionId,
    GroupId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LogicalChildren::Table)
                    .if_not_exists()
                    .col(integer(LogicalChildren::LogicalExpressionId))
                    .col(integer(LogicalChildren::GroupId))
                    .primary_key(
                        Index::create()
                            .col(LogicalChildren::LogicalExpressionId)
                            .col(LogicalChildren::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalChildren::Table, LogicalChildren::GroupId)
                            .to(LogicalExpression::Table, LogicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalChildren::Table, LogicalChildren::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LogicalChildren::Table).to_owned())
            .await
    }
}
