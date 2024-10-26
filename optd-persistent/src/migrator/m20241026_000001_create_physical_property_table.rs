use super::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalProperty {
    Table,
    Id,
    Data,
    PhysicalExpressionId,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241026_000001_create_physical_property_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // TODO add property variant identifier
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhysicalProperty::Table)
                    .if_not_exists()
                    .col(pk_auto(PhysicalProperty::Id))
                    .col(json(PhysicalProperty::Data))
                    .col(integer(PhysicalProperty::PhysicalExpressionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_property-physical_expression")
                            .from(
                                PhysicalProperty::Table,
                                PhysicalProperty::PhysicalExpressionId,
                            )
                            .to(PhysicalExpression::Table, PhysicalExpression::Id),
                    )
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
