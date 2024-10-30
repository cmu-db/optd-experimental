use super::constraint::Constraint;
use super::table_attribute::TableAttribute;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::integer;

#[derive(Iden)]
pub enum ForeignConstraintRefAttributeJunction {
    Table,
    ConstraintId,
    AttrId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ForeignConstraintRefAttributeJunction::Table)
                    .if_not_exists()
                    .col(integer(ForeignConstraintRefAttributeJunction::ConstraintId))
                    .col(integer(ForeignConstraintRefAttributeJunction::AttrId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ForeignConstraintRefAttributeJunction::Table,
                                ForeignConstraintRefAttributeJunction::ConstraintId,
                            )
                            .to(Constraint::Table, Constraint::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ForeignConstraintRefAttributeJunction::Table,
                                ForeignConstraintRefAttributeJunction::AttrId,
                            )
                            .to(TableAttribute::Table, TableAttribute::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(ForeignConstraintRefAttributeJunction::ConstraintId)
                            .col(ForeignConstraintRefAttributeJunction::AttrId),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ForeignConstraintRefAttributeJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
