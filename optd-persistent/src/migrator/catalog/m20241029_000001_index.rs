use crate::migrator::catalog::table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Index {
    Table,
    Id,
    TableId,
    Name,
    NumberOfAttributes,
    VariantTag,
    IsUnique,
    NullsNotDistinct,
    IsPrimary,
    IsClustered,
    IsExclusion,
    Description,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Index::Table)
                    .if_not_exists()
                    .col(pk_auto(Index::Id))
                    .col(integer(Index::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Index::Table, Index::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Index::Name))
                    .col(integer(Index::NumberOfAttributes))
                    .col(integer(Index::VariantTag))
                    .col(boolean(Index::IsUnique))
                    .col(boolean(Index::NullsNotDistinct))
                    .col(boolean(Index::IsPrimary))
                    .col(boolean(Index::IsClustered))
                    .col(boolean(Index::IsExclusion))
                    .col(json(Index::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Index::Table).to_owned())
            .await
    }
}
