use super::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalProperty {
    Table,
    Id,
    Data,
    GroupId,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_logical_property"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // TODO add property variant identifier
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LogicalProperty::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalProperty::Id))
                    .col(json(LogicalProperty::Data))
                    .col(integer(LogicalProperty::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-logical_property-group_id")
                            .from(LogicalProperty::Table, LogicalProperty::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id),
                    )
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
