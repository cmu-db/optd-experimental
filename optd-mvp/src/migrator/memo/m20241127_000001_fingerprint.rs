//! An entity representing a logical expression fingerprint.
//!
//! TODO write docs.

use crate::migrator::memo::logical_expression::LogicalExpression;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Fingerprint {
    Table,
    Id,
    LogicalExpressionId,
    Kind,
    Hash,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Fingerprint::Table)
                    .if_not_exists()
                    .col(pk_auto(Fingerprint::Id))
                    .col(unsigned(Fingerprint::LogicalExpressionId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Fingerprint::Table, Fingerprint::LogicalExpressionId)
                            .to(LogicalExpression::Table, LogicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(small_unsigned(Fingerprint::Kind))
                    .col(big_unsigned(Fingerprint::Hash))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Fingerprint::Table).to_owned())
            .await
    }
}
