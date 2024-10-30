use super::cascades_group::CascadesGroup;
use super::cost::Cost;
use super::event::Event;
use super::physical_expression::PhysicalExpression;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

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
                    .col(pk_auto(GroupWinner::Id))
                    .col(integer(GroupWinner::GroupId))
                    .col(integer(GroupWinner::PhysicalExpressionId))
                    .col(integer(GroupWinner::Cost))
                    .col(integer(GroupWinner::EpochId))
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
