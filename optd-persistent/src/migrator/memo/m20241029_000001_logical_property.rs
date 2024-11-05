//! An entity representing a logical property of a Cascades group.
//!
//! TODO what exactly are we storing in here?
//! TODO why is it linked to only cascades groups and not logical expressions?

use crate::migrator::memo::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalProperty {
    Table,
    Id,
    GroupId,
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
                    .table(LogicalProperty::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalProperty::Id))
                    .col(integer(LogicalProperty::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalProperty::Table, LogicalProperty::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(small_integer(LogicalProperty::VariantTag))
                    .col(json(LogicalProperty::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LogicalProperty::Table).to_owned())
            .await
    }
}
