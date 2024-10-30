use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum DatabaseMetadata {
    Table,
    Id,
    CreatedTime,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DatabaseMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DatabaseMetadata::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(DatabaseMetadata::CreatedTime).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DatabaseMetadata::Table).to_owned())
            .await
    }
}
