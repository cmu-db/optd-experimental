use crate::migrator::catalog::table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Trigger {
    Table,
    Id,
    Name,
    TableId,
    ParentTriggerId,
    Function,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Trigger::Table)
                    .if_not_exists()
                    .col(pk_auto(Trigger::Id))
                    .col(string(Trigger::Name))
                    .col(integer(Trigger::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Trigger::Table, Trigger::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Trigger::ParentTriggerId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Trigger::Table, Trigger::ParentTriggerId)
                            .to(Trigger::Table, Trigger::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(json(Trigger::Function))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Trigger::Table).to_owned())
            .await
    }
}
