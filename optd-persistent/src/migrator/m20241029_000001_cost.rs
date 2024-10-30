/*
Table cost {
  id integer PK
  expr_id integer [ref: > physical_expression.id]
  epoch_id integer [ref: > event.epoch_id]
  cost integer
  valid boolean
} */

use super::event::Event;
use super::physical_expression::PhysicalExpression;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::pk_auto;
use sea_orm_migration::schema::*;

#[derive(Iden)]
pub enum Cost {
    Table,
    Id,
    ExprId,
    EpochId,
    Cost,
    Valid,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cost::Table)
                    .if_not_exists()
                    .col(pk_auto(Cost::Id))
                    .col(integer(Cost::ExprId))
                    .col(integer(Cost::EpochId))
                    .col(integer(Cost::Cost))
                    .col(boolean(Cost::Valid))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cost::Table, Cost::ExprId)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cost::Table, Cost::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cost::Table).to_owned())
            .await
    }
}
