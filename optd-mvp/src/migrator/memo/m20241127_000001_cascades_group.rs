//! An entity representing a group / equivalence class in the Cascades framework.
//!
//! Quoted from the Microsoft article _Extensible query optimizers in practice_:
//!
//! > In the memo, each class of equivalent expressions is called an equivalent class or a group,
//! > and all equivalent expressions within the class are called group expressions or simply
//! > expressions.
//!
//! A Cascades group is defined as a class of equivalent logical or physical expressions. The
//! Cascades framework uses these groups as a way of storing the best query sub-plans for use in the
//! dynamic programming search algorithm.
//!
//! For example, a Cascades group could be the set of expressions containing the logical expressions
//! `Join(A, B)` and `Join(B, A)`, as well as the physical expressions `HashJoin(A, B)` and
//! `NestedLoopJoin(B, A)`.
//!
//! # Columns
//!
//! Each group is assigned a monotonically-increasing (unique) ID. This ID will be important since
//! there are many foreign key references from other tables to `cascades_group`.
//!
//! We additionally store a `latest_winner` foreign key reference to a physical expression. See
//! the [section](#best-physical-plan-winner) below for more details.
//!
//! Finally, we store an `is_optimized` flag that is used for quickly determining the state of
//! optimization for this group during the dynamic programming search.
//!
//! # Entity Relationships
//!
//! ### Child Expressions (Logical and Physical)
//!
//! To retrieve all of a `cascades_group`'s equivalent expressions, you must query the
//! [`logical_expression`] or the [`physical_expression`] entities via their foreign keys to
//! `cascades_group`. The relationship between [`logical_expression`] and `cascades_group` is
//! many-to-one, and the exact same many-to-one relationship is held for [`physical_expression`] to
//! `cascades_group`.
//!
//! ### Parent Expressions (Logical and Physical)
//!
//! Additionally, each logical or physical expression can have any number of `cascades_group`s as
//! children, and a group can be a child of any expression. Thus, `cascades_group` additionally has
//! a many-to-many relationship with [`logical_expression`] and [`physical_expression`] via the
//! [`logical_children`] and [`physical_children`] entities.
//!
//! To reiterate, `cascades_group` has **both** a one-to-many **and** a many-to-many relationship
//! with both [`logical_expression`] and [`physical_expression`]. This is due to groups being both
//! parents and children of expressions.
//!
//! ### Best Physical Plan (Winner)
//!
//! The `cascades_group` entity also stores a `latest_winner` _nullable_ foreign key reference to
//! a physical expression. This represents the most recent best query plan we have computed. The
//! reason it is nullable is because we may not have come up with any best query plan yet.
//!
//! ### Logical Properties
//!
//! FIXME: Add a logical properties table.
//!
//! Lastly, each `cascades_group` record will have a set of logical properties store in the
//! `logical_property` entity, where there is an many-to-one relationship from
//! `logical_property` to `cascades_group`. Note that we do not store physical properties directly
//! on the `cascades_group`, but rather we store them for each [`physical_expression`] record.
//!
//! [`logical_expression`]: super::logical_expression
//! [`physical_expression`]: super::physical_expression
//! [`logical_children`]: super::logical_children
//! [`physical_children`]: super::physical_children
//! `logical_property`: super::logical_property

use crate::migrator::memo::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum CascadesGroup {
    Table,
    Id,
    Status,
    Winner,
    Cost,
    ParentId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CascadesGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(CascadesGroup::Id))
                    .col(tiny_integer(CascadesGroup::Status))
                    .col(integer_null(CascadesGroup::Winner))
                    .col(big_integer_null(CascadesGroup::Cost))
                    .foreign_key(
                        ForeignKey::create()
                            .from(CascadesGroup::Table, CascadesGroup::Winner)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(CascadesGroup::ParentId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(CascadesGroup::Table, CascadesGroup::ParentId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CascadesGroup::Table).to_owned())
            .await
    }
}
