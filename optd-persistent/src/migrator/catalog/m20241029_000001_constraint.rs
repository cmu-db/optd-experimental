use crate::migrator::catalog::{index::Index, table_metadata::TableMetadata};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum Constraint {
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
                    .table(Constraint::Table)
                    .if_not_exists()
                    .col(pk_auto(Constraint::Id))
                    .col(string(Constraint::Name))
                    .col(integer(Constraint::VariantTag))
                    .col(integer_null(Constraint::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Constraint::Table, Constraint::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Constraint::IndexId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Constraint::Table, Constraint::IndexId)
                            .to(Index::Table, Index::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Constraint::ForeignRefId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Constraint::Table, Constraint::ForeignRefId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Constraint::CheckSrc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Constraint::Table).to_owned())
            .await
    }
}
