use crate::migrator::memo::{
    cascades_group::CascadesGroup, physical_expression::PhysicalExpression,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalGroupJunction {
    Table,
    GroupId,
    PhysicalExpressionId,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_physical_group_junction"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhysicalGroupJunction::Table)
                    .if_not_exists()
                    .col(integer(PhysicalGroupJunction::GroupId))
                    .col(integer(PhysicalGroupJunction::PhysicalExpressionId))
                    .primary_key(
                        Index::create()
                            .col(PhysicalGroupJunction::GroupId)
                            .col(PhysicalGroupJunction::PhysicalExpressionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_group_junction-group_id")
                            .from(PhysicalGroupJunction::Table, PhysicalGroupJunction::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_group_junction-physical_expression")
                            .from(
                                PhysicalGroupJunction::Table,
                                PhysicalGroupJunction::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
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
