use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum DatabaseMetadata {
    Table,
    Id,
    CreateTime,
    Name,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241029_000001_database_metadata"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DatabaseMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(DatabaseMetadata::Id))
                    .col(timestamp(DatabaseMetadata::CreateTime).default("CURRENT_TIMESTAMP"))
                    .col(string(DatabaseMetadata::Name).not_null())
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
