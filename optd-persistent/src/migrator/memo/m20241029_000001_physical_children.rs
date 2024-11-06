use crate::migrator::memo::{
    cascades_group::CascadesGroup, physical_expression::PhysicalExpression,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalChildren {
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
                    .table(PhysicalChildren::Table)
                    .if_not_exists()
                    .col(integer(PhysicalChildren::PhysicalExpressionId))
                    .col(integer(PhysicalChildren::GroupId))
                    .primary_key(
                        Index::create()
                            .col(PhysicalChildren::PhysicalExpressionId)
                            .col(PhysicalChildren::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PhysicalChildren::Table,
                                PhysicalChildren::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhysicalChildren::Table, PhysicalChildren::GroupId)
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
            .drop_table(Table::drop().table(PhysicalChildren::Table).to_owned())
            .await
    }
}
