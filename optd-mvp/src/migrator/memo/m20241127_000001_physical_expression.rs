//! An entity representing a logical plan expression in the Cascades framework.
//!
//! Quoted from the Microsoft article _Extensible query optimizers in practice_:
//!
//! > A physical expression is a tree of physical operators, which is also referred to as the
//! > _physical plan_ or simply _plan_.
//!
//! In the Cascades query optimization framework, the memo table stores equivalence classes of
//! expressions (see [`cascades_group`]). These equivalence classes, or "groups", store both
//! [`logical_expression`]s and `physical_expression`s.
//!
//! Optimization starts by exploring equivalent logical expressions within a group, and then it
//! proceeds to implement / optimize those logical operators into physical operators. For example,
//! the logical expression `Join(A, B)` could be implemented into a `HashJoin(A, B)` or a
//! `NestedLoopJoin(A, B)`, and both of these new physical expressions would be contained in the
//! same group.
//!
//! # Columns
//!
//! Each `physical_expression` has a unique primary key ID, and other tables will store a foreign
//! key reference to a specific `physical_expression`s.
//!
//! The more interesting column is the `fingerprint` column, in which we store a hashed fingerprint
//! value that can be used to efficiently check equality between two potentially equivalent physical
//! expressions (hash-consing). See ???FIXME??? for more information on expression fingerprints.
//!
//! Finally, since there are many different types of operators, we store a variant tag and a data
//! column as JSON to represent the semi-structured data fields of logical operators.
//!
//! # Entity Relationships
//!
//! The only relationship that `physical_expression` has is to [`cascades_group`]. It has **both** a
//! one-to-many **and** a many-to-many relationship with [`cascades_group`], and you can see more
//! details about this in the module-level documentation for [`cascades_group`].
//!
//! [`cascades_group`]: super::cascades_group
//! [`logical_expression`]: super::logical_expression

use crate::migrator::memo::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalExpression {
    Table,
    Id,
    GroupId,
    Fingerprint,
    Kind,
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
                    .table(PhysicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(PhysicalExpression::Id))
                    .col(integer(PhysicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhysicalExpression::Table, PhysicalExpression::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(PhysicalExpression::Fingerprint))
                    .col(small_integer(PhysicalExpression::Kind))
                    .col(json(PhysicalExpression::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhysicalExpression::Table).to_owned())
            .await
    }
}
