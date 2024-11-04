use crate::migrator::memo::{
    cascades_group::CascadesGroup, physical_expression::PhysicalExpression,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalGroupJunction {
    Table,
    PhysicalExpressionId,
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
                    .table(PhysicalGroupJunction::Table)
                    .if_not_exists()
                    .col(integer(PhysicalGroupJunction::PhysicalExpressionId))
                    .col(integer(PhysicalGroupJunction::GroupId))
                    .primary_key(
                        Index::create()
                            .col(PhysicalGroupJunction::PhysicalExpressionId)
                            .col(PhysicalGroupJunction::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PhysicalGroupJunction::Table,
                                PhysicalGroupJunction::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhysicalGroupJunction::Table, PhysicalGroupJunction::GroupId)
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
            .drop_table(Table::drop().table(PhysicalGroupJunction::Table).to_owned())
            .await
    }
}
