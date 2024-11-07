//! An entity representing the relationship between [`attribute`] and [`constraint`].
//!
//! If a constraint is a table constraint (including foreign keys, but not constraint triggers),
//! the attributes that are constrained on are stored in the [`attribute_constraint_junction`].
//!
//! One constraint might be associated with multiple attributes, for example, a composite primary key.

use crate::migrator::catalog::{attribute::Attribute, constraint::Constraint};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum AttributeConstraintJunction {
    Table,
    AttributeId,
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
                    .table(AttributeConstraintJunction::Table)
                    .if_not_exists()
                    .col(integer(AttributeConstraintJunction::AttributeId))
                    .col(integer(AttributeConstraintJunction::ConstraintId))
                    .primary_key(
                        Index::create()
                            .col(AttributeConstraintJunction::AttributeId)
                            .col(AttributeConstraintJunction::ConstraintId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeConstraintJunction::Table,
                                AttributeConstraintJunction::AttributeId,
                            )
                            .to(Attribute::Table, Attribute::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeConstraintJunction::Table,
                                AttributeConstraintJunction::ConstraintId,
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
                    .table(AttributeConstraintJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
