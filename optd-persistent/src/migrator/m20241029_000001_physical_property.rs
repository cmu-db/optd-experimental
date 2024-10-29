use super::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalProperty {
    Table,
    Id,
    PhysicalExpressionId,
    VariantTag,
    Data,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_physical_property"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Note that the foreign key constraint is `Cascade` for both delete and update, since if
        // for some reason the physical expression ID (primary key) changes, we want to update the
        // foreign keys of the physical properties that reference it.
        manager
            .create_table(
                Table::create()
                    .table(PhysicalProperty::Table)
                    .if_not_exists()
                    .col(pk_auto(PhysicalProperty::Id))
                    .col(integer(PhysicalProperty::PhysicalExpressionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_property-physical_expression")
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
