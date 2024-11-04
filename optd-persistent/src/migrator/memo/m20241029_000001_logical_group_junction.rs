use crate::migrator::memo::{cascades_group::CascadesGroup, logical_expression::LogicalExpression};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalGroupJunction {
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
                    .table(LogicalGroupJunction::Table)
                    .if_not_exists()
                    .col(integer(LogicalGroupJunction::LogicalExpressionId))
                    .col(integer(LogicalGroupJunction::GroupId))
                    .primary_key(
                        Index::create()
                            .col(LogicalGroupJunction::LogicalExpressionId)
                            .col(LogicalGroupJunction::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalGroupJunction::Table, LogicalGroupJunction::GroupId)
                            .to(LogicalExpression::Table, LogicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalGroupJunction::Table, LogicalGroupJunction::GroupId)
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
            .drop_table(Table::drop().table(LogicalGroupJunction::Table).to_owned())
            .await
    }
}
