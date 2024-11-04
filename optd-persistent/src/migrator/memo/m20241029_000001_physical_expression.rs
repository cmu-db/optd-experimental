use crate::migrator::memo::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalExpression {
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
                    .table(PhysicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(PhysicalExpression::Id))
                    .col(integer(PhysicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhysicalExpression::Table, PhysicalExpression::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(PhysicalExpression::Fingerprint))
                    .col(small_integer(PhysicalExpression::VariantTag))
                    .col(json(PhysicalExpression::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhysicalExpression::Table).to_owned())
            .await
    }
}
