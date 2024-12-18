//! An entity representing a logical plan expression in the Cascades framework.
//!
//! Quoted from the Microsoft article _Extensible query optimizers in practice_:
//!
//! > A logical expression is defined as a tree of logical operators, and corresponds to a
//! > relational algebraic expression.
//!
//! In the Cascades query optimization framework, the memo table stores equivalence classes of
//! expressions (see [`cascades_group`]). These equivalence classes, or "groups", store both
//! `logical_expression`s and [`physical_expression`]s.
//!
//! Optimization starts by "exploring" equivalent logical expressions within a group. For example,
//! the logical expressions `Join(A, B)` and `Join(B, A)` are contained in the same group. The
//! logical expressions are defined as a `Join` operator with the groups representing a scan of
//! table `A` and a scan of table `B` as its children.
//!
//! # Columns
//!
//! Each `logical_expression` has a unique primary key ID, but it holds little importance other than
//! helping distinguish between two different expressions.
//!
//! The more interesting column is the `fingerprint` column, in which we store a hashed fingerprint
//! value that can be used to efficiently check equality between two potentially equivalent logical
//! expressions (hash-consing). See ???TODO??? for more information on expression fingerprints.
//!
//! Finally, since there are many different types of operators, we store a variant tag and a data
//! column as JSON to represent the semi-structured data fields of logical operators.
//!
//! # Entity Relationships
//!
//! The only relationship that `logical_expression` has is to [`cascades_group`]. It has **both** a
//! one-to-many **and** a many-to-many relationship with [`cascades_group`], and you can see more
//! details about this in the module-level documentation for [`cascades_group`].
//!
//! [`cascades_group`]: super::cascades_group
//! [`physical_expression`]: super::physical_expression

use crate::migrator::memo::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalExpression {
    Table,
    Id,
    GroupId,
    Fingerprint,
    VariantTag,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LogicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalExpression::Id))
                    .col(integer(LogicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalExpression::Table, LogicalExpression::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(LogicalExpression::Fingerprint))
                    .col(small_integer(LogicalExpression::VariantTag))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LogicalExpression::Table).to_owned())
            .await
    }
}
