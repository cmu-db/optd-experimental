use crate::migrator::catalog::{attribute::Attribute, constraint::Constraint};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum AttributeForeignConstraintJunction {
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
                    .table(AttributeForeignConstraintJunction::Table)
                    .if_not_exists()
                    .col(integer(AttributeForeignConstraintJunction::AttributeId))
                    .col(integer(AttributeForeignConstraintJunction::ConstraintId))
                    .primary_key(
                        Index::create()
                            .col(AttributeForeignConstraintJunction::AttributeId)
                            .col(AttributeForeignConstraintJunction::ConstraintId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeForeignConstraintJunction::Table,
                                AttributeForeignConstraintJunction::AttributeId,
                            )
                            .to(Attribute::Table, Attribute::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeForeignConstraintJunction::Table,
                                AttributeForeignConstraintJunction::ConstraintId,
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
                    .table(AttributeForeignConstraintJunction::Table)
                    .to_owned(),
            )
            .await
    }
}