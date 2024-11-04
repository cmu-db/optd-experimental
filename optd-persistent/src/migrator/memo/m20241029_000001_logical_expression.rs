use crate::migrator::memo::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalExpression {
    Table,
    Id,
    GroupId,
    Fingerprint,
    VariantTag,
    Data,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LogicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalExpression::Id))
                    .col(integer(LogicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalExpression::Table, LogicalExpression::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(LogicalExpression::Fingerprint))
                    .col(small_integer(LogicalExpression::VariantTag))
                    .col(json(LogicalExpression::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LogicalExpression::Table).to_owned())
            .await
    }
}
