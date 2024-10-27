use super::{cascades_group::CascadesGroup, logical_expression::LogicalExpression};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalGroupJunction {
    Table,
    GroupId,
    LogicalExpressionId,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241026_000001_create_logical_group_junction_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LogicalGroupJunction::Table)
                    .if_not_exists()
                    .col(integer(LogicalGroupJunction::GroupId))
                    .col(integer(LogicalGroupJunction::LogicalExpressionId))
                    .primary_key(
                        Index::create()
                            .col(LogicalGroupJunction::GroupId)
                            .col(LogicalGroupJunction::LogicalExpressionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_group_junction-group_id")
                            .from(LogicalGroupJunction::Table, LogicalGroupJunction::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_group_junction-logical_expression")
                            .from(LogicalGroupJunction::Table, LogicalGroupJunction::GroupId)
                            .to(LogicalExpression::Table, LogicalExpression::Id),
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
