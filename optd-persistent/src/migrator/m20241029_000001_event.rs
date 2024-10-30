use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Event {
    Table,
    Id,
    EpochId,
    SourceVariant,
    CreateTimestamp,
    Data,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Event::Id).integer().primary_key().auto_increment())
                    .col(ColumnDef::new(Event::EpochId).integer())
                    .col(ColumnDef::new(Event::SourceVariant).string())
                    .col(ColumnDef::new(Event::CreateTimestamp).timestamp())
                    .col(ColumnDef::new(Event::Data).json())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}
