use super::cascades_group::CascadesGroup;
use super::cost::Cost;
use super::event::Event;
use super::physical_expression::PhysicalExpression;
use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum GroupWinner {
    Table,
    Id,
    GroupId,
    PhysicalExpressionId,
    Cost,
    EpochId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GroupWinner::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GroupWinner::Id).integer().primary_key().auto_increment())
                    .col(ColumnDef::new(GroupWinner::GroupId).integer())
                    .col(ColumnDef::new(GroupWinner::PhysicalExpressionId).integer())
                    .col(ColumnDef::new(GroupWinner::Cost).integer())
                    .col(ColumnDef::new(GroupWinner::EpochId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::EpochId)
                            .to(Event::Table, Event::EpochId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::PhysicalExpressionId)
                            .to(PhysicalExpression::Table, PhysicalExpression::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupWinner::Table, GroupWinner::Cost)
                            .to(Cost::Table, Cost::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GroupWinner::Table).to_owned())
            .await
    }
}
