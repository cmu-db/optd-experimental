//! FIXME We aren't really following the cascades framework anymore...
//!
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
//! there are many foreign key references from other tables to `group`.
//!
//! We store an `status` enumeration encoded as an 8-bit integer that is used for quickly
//! determining the state of optimization for this group during the dynamic programming search.
//!
//! We additionally store a `winner` foreign key reference to a physical expression paired with a
//! `cost` foreign key reference to a cost record (FIXME). See the
//! [section](#best-physical-plan-winner) below for more details.
//!
//! Finally, we maintain a union-find graph structure embedded in the group records.
//! TODO write more information about this once this is implemented.
//!
//! # Entity Relationships
//!
//! ### Child Expressions (Logical and Physical)
//!
//! To retrieve all of a `group`'s equivalent expressions, you must query the
//! [`logical_expression`] or the [`physical_expression`] entities via their foreign keys to
//! `group`. The relationship between [`logical_expression`] and `group` is
//! many-to-one, and the exact same many-to-one relationship is held for [`physical_expression`] to
//! `group`.
//!
//! ### Parent Expressions (Logical and Physical)
//!
//! Additionally, each logical or physical expression can have any number of `group`s as
//! children, and a group can be a child of any expression. Thus, `group` additionally has
//! a many-to-many relationship with [`logical_expression`] and [`physical_expression`] via the
//! [`logical_children`] and [`physical_children`] entities.
//!
//! To reiterate, `group` has **both** a one-to-many **and** a many-to-many relationship
//! with both [`logical_expression`] and [`physical_expression`]. This is due to groups being both
//! parents and children of expressions.
//!
//! ### Best Physical Plan (Winner)
//!
//! The `group` entity also stores a `winner` _nullable_ foreign key reference to
//! a physical expression. This represents the most recent best query plan we have computed. The
//! reason it is nullable is because we may not have come up with any best query plan yet.
//!
//! ### Logical Properties
//!
//! FIXME: Add a logical properties table.
//!
//! Lastly, each `group` record will have a set of logical properties store in the
//! `logical_property` entity, where there is an many-to-one relationship from
//! `logical_property` to `group`. Note that we do not store physical properties directly
//! on the `group`, but rather we store them for each [`physical_expression`] record.
//!
//! [`logical_expression`]: super::logical_expression
//! [`physical_expression`]: super::physical_expression
//! [`logical_children`]: super::logical_children
//! [`physical_children`]: super::physical_children
//! `logical_property`: super::logical_property

use crate::migrator::memo::physical_expression::PhysicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Group {
    Table,
    Id,
    Status,
    Winner,
    Cost,
    ParentId,
    NextId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(pk_auto(Group::Id))
                    .col(tiny_integer(Group::Status))
                    .col(integer_null(Group::Winner))
                    .col(big_integer_null(Group::Cost))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Group::Table, Group::Winner)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Group::ParentId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Group::Table, Group::ParentId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Group::NextId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Group::Table, Group::NextId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}
