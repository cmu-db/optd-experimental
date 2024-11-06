//! An entity representing a physical property of a physical expression in the Cascades framework.
//!
//! TODO what exactly are we storing in here?
//! TODO why is it linked to only physical expressions and not cascades groups?

use crate::migrator::memo::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalProperty {
    Table,
    Id,
    PhysicalExpressionId,
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
                    .table(PhysicalProperty::Table)
                    .if_not_exists()
                    .col(pk_auto(PhysicalProperty::Id))
                    .col(integer(PhysicalProperty::PhysicalExpressionId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PhysicalProperty::Table,
                                PhysicalProperty::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(small_integer(PhysicalProperty::VariantTag))
                    .col(json(PhysicalProperty::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhysicalProperty::Table).to_owned())
            .await
    }
}
