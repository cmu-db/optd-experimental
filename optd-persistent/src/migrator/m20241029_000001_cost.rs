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
                    .col(ColumnDef::new(Cost::Id).integer().primary_key().auto_increment())
                    .col(ColumnDef::new(Cost::ExprId).integer())
                    .col(ColumnDef::new(Cost::EpochId).integer())
                    .col(ColumnDef::new(Cost::Cost).integer())
                    .col(ColumnDef::new(Cost::Valid).boolean())
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
                            .to(Event::Table, Event::Id)
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
