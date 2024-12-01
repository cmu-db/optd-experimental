//! An entity representing a logical relational expression.
//!
//! Quoted from the Microsoft article _Extensible query optimizers in practice_:
//!
//! > A logical expression is defined as a tree of logical operators, and corresponds to a
//! > relational algebraic expression.
//!
//! In the Cascades query optimization framework, the memo table stores equivalence classes of
//! expressions (see [`group`]). These equivalence classes, or "groups", store both
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
//! Finally, since there are many different types of operators, we store a variant tag and a data
//! column as JSON to represent the semi-structured data fields of logical operators.
//!
//! # Entity Relationships
//!
//! The main relationship that `logical_expression` has is to [`group`]. It has **both** a
//! one-to-many **and** a many-to-many relationship with [`group`], and you can see more
//! details about this in the module-level documentation for [`group`].
//!
//! The other relationship that `logical_expression` has is to [`fingerprint`]. This table stores
//! 1 or more fingerprints for every (unique) logical expression. The reason we have multiple
//! fingerprints is that an expression can belong to multiple groups during the exploration phase
//! before the merging of groups.
//!
//! [`group`]: super::group
//! [`physical_expression`]: super::physical_expression
//! [`fingerprint`]: super::fingerprint

use crate::migrator::memo::group::Group;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum LogicalExpression {
    Table,
    Id,
    GroupId,
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
                    .table(LogicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(LogicalExpression::Id))
                    .col(integer(LogicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(LogicalExpression::Table, LogicalExpression::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(small_integer(LogicalExpression::Kind))
                    .col(json(LogicalExpression::Data))
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
