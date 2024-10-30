/*
// The constrained attributes (columns) if a constraint is a table constraint (including foreign keys, but not constraint triggers)
Table constraint_attribute_junction {
  constraint_id integer [ref: > constraint.id]
  attr_id integer [ref: > table_attribute.id]
}
*/

use super::constraint::Constraint;
use super::table_attribute::TableAttribute;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(Iden)]
pub enum ConstraintAttributeJunction {
    Table,
    ConstraintId,
    AttrId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConstraintAttributeJunction::Table)
                    .if_not_exists()
                    .col(integer(ConstraintAttributeJunction::ConstraintId))
                    .col(integer(ConstraintAttributeJunction::AttrId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ConstraintAttributeJunction::Table,
                                ConstraintAttributeJunction::ConstraintId,
                            )
                            .to(Constraint::Table, Constraint::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ConstraintAttributeJunction::Table,
                                ConstraintAttributeJunction::AttrId,
                            )
                            .to(TableAttribute::Table, TableAttribute::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(ConstraintAttributeJunction::ConstraintId)
                            .col(ConstraintAttributeJunction::AttrId)
                            .name("constraint_attribute_junction_pk")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ConstraintAttributeJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
