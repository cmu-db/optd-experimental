/*
Table predicate_physical_expression_junction {
  physical_expr_id integer [ref: > physical_expression.id]
  predicate_id integer [ref: > predicate.id]
}
 */

use sea_orm_migration::{prelude::*, schema::integer};

use super::{
    m20241029_000001_physical_expression::PhysicalExpression, m20241029_000001_predicate::Predicate,
};

#[derive(Iden)]
pub enum PredicatePhysicalExpressionJunction {
    Table,
    PhysicalExprId,
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
                    .table(PredicatePhysicalExpressionJunction::Table)
                    .col(integer(PredicatePhysicalExpressionJunction::PhysicalExprId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("predicate_physical_expression_junction_physical_expr_id_fkey")
                            .from(
                                PredicatePhysicalExpressionJunction::Table,
                                PredicatePhysicalExpressionJunction::PhysicalExprId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PredicatePhysicalExpressionJunction::PredicateId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("predicate_physical_expression_junction_predicate_id_fkey")
                            .from(
                                PredicatePhysicalExpressionJunction::Table,
                                PredicatePhysicalExpressionJunction::PredicateId,
                            )
                            .to(Predicate::Table, Predicate::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(PredicatePhysicalExpressionJunction::PhysicalExprId)
                            .col(PredicatePhysicalExpressionJunction::PredicateId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PredicatePhysicalExpressionJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
