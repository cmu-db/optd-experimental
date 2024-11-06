/*
Table predicate_logical_expression_junction {
  logical_expr_id integer [ref: > logical_expression.id]
  predicate_id integer [ref: > predicate.id]
}
 */

use sea_orm_migration::{prelude::*, schema::integer};

use super::{
    m20241029_000001_logical_expression::LogicalExpression, m20241029_000001_predicate::Predicate,
};

#[derive(Iden)]
pub enum PredicateLogicalExpressionJunction {
    Table,
    LogicalExprId,
    PredicateId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PredicateLogicalExpressionJunction::Table)
                    .col(integer(PredicateLogicalExpressionJunction::LogicalExprId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PredicateLogicalExpressionJunction::Table,
                                PredicateLogicalExpressionJunction::LogicalExprId,
                            )
                            .to(LogicalExpression::Table, LogicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PredicateLogicalExpressionJunction::PredicateId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PredicateLogicalExpressionJunction::Table,
                                PredicateLogicalExpressionJunction::PredicateId,
                            )
                            .to(Predicate::Table, Predicate::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(PredicateLogicalExpressionJunction::LogicalExprId)
                            .col(PredicateLogicalExpressionJunction::PredicateId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PredicateLogicalExpressionJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
