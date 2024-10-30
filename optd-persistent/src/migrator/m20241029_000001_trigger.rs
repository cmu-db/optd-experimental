use super::table_metadata::TableMetadata;
use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Trigger {
    Table,
    Id,
    TableId,
    Name,
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
                    .col(
                        ColumnDef::new(Trigger::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Trigger::TableId).integer())
                    .col(ColumnDef::new(Trigger::Name).string())
                    .col(ColumnDef::new(Trigger::ParentTriggerId).integer())
                    .col(ColumnDef::new(Trigger::Function).json())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Trigger::Table, Trigger::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Trigger::Table, Trigger::ParentTriggerId)
                            .to(Trigger::Table, Trigger::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
