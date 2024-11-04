use crate::migrator::memo::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum CascadesGroup {
    Table,
    Id,
    LatestWinner,
    InProgress,
    IsOptimized,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CascadesGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(CascadesGroup::Id))
                    .col(integer_null(CascadesGroup::LatestWinner))
                    .foreign_key(
                        ForeignKey::create()
                            .from(CascadesGroup::Table, CascadesGroup::LatestWinner)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(boolean(CascadesGroup::InProgress))
                    .col(boolean(CascadesGroup::IsOptimized))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CascadesGroup::Table).to_owned())
            .await
    }
}
