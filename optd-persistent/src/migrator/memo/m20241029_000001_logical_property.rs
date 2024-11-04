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

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_logical_property"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Note that the foreign key constraint is `Cascade` for both delete and update, since if
        // for some reason the group ID (primary key) changes, we want to update the foreign keys of
        // the logical properties that reference it.
        manager
            .create_table(
                Table::create()
                    .table(LogicalProperty::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalProperty::Id))
                    .col(integer(LogicalProperty::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_property-group_id")
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
