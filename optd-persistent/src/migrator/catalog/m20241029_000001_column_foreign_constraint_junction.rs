use crate::migrator::catalog::{column::Column, constraint::Constraint};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum ColumnForeignConstraintJunction {
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
                    .table(ColumnForeignConstraintJunction::Table)
                    .if_not_exists()
                    .col(integer(ColumnForeignConstraintJunction::ColumnId))
                    .col(integer(ColumnForeignConstraintJunction::ConstraintId))
                    .primary_key(
                        Index::create()
                            .col(ColumnForeignConstraintJunction::ColumnId)
                            .col(ColumnForeignConstraintJunction::ConstraintId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ColumnForeignConstraintJunction::Table,
                                ColumnForeignConstraintJunction::ColumnId,
                            )
                            .to(Column::Table, Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ColumnForeignConstraintJunction::Table,
                                ColumnForeignConstraintJunction::ConstraintId,
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
                    .table(ColumnForeignConstraintJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
