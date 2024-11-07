use crate::migrator::catalog::table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum IndexMetadata {
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
                    .table(IndexMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(IndexMetadata::Id))
                    .col(integer(IndexMetadata::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(IndexMetadata::Table, IndexMetadata::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(IndexMetadata::Name))
                    .col(integer(IndexMetadata::NumberOfAttributes))
                    .col(integer(IndexMetadata::VariantTag))
                    .col(boolean(IndexMetadata::IsUnique))
                    .col(boolean(IndexMetadata::NullsNotDistinct))
                    .col(boolean(IndexMetadata::IsPrimary))
                    .col(boolean(IndexMetadata::IsClustered))
                    .col(boolean(IndexMetadata::IsExclusion))
                    .col(string(IndexMetadata::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IndexMetadata::Table).to_owned())
            .await
    }
}
