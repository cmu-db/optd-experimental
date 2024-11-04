/*
// The constrained attributes (columns) if a constraint is a table constraint (including foreign keys, but not constraint triggers)
Table constraint_attribute_junction {
  constraint_id integer [ref: > constraint.id]
  attr_id integer [ref: > table_attribute.id]
}
*/

use crate::migrator::catalog::{column::Column, constraint::Constraint};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum ColumnConstraintJunction {
    Table,
    ColumnId,
    ConstraintId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ColumnConstraintJunction::Table)
                    .if_not_exists()
                    .col(integer(ColumnConstraintJunction::ColumnId))
                    .col(integer(ColumnConstraintJunction::ConstraintId))
                    .primary_key(
                        Index::create()
                            .col(ColumnConstraintJunction::ColumnId)
                            .col(ColumnConstraintJunction::ConstraintId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ColumnConstraintJunction::Table,
                                ColumnConstraintJunction::ColumnId,
                            )
                            .to(Column::Table, Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ColumnConstraintJunction::Table,
                                ColumnConstraintJunction::ConstraintId,
                            )
                            .to(Constraint::Table, Constraint::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ColumnConstraintJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
