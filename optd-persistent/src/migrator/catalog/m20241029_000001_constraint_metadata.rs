use crate::migrator::catalog::{index_metadata::IndexMetadata, table_metadata::TableMetadata};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum ConstraintMetadata {
    Table,
    Id,
    Name,
    VariantTag,
    TableId,
    IndexId,
    ForeignRefId,
    CheckSrc,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConstraintMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(ConstraintMetadata::Id))
                    .col(string(ConstraintMetadata::Name))
                    .col(integer(ConstraintMetadata::VariantTag))
                    .col(integer_null(ConstraintMetadata::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ConstraintMetadata::Table, ConstraintMetadata::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(ConstraintMetadata::IndexId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ConstraintMetadata::Table, ConstraintMetadata::IndexId)
                            .to(IndexMetadata::Table, IndexMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(ConstraintMetadata::ForeignRefId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ConstraintMetadata::Table, ConstraintMetadata::ForeignRefId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(ConstraintMetadata::CheckSrc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConstraintMetadata::Table).to_owned())
            .await
    }
}
