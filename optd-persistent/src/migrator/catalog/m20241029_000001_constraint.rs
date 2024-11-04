/*
// Not-null is handled directly in `table_attribute`. See `is_not_null` field.
// Constraint trigger is handled directly in `trigger`.
Table constraint {
  id integer PK
  name varchar
  constraint_type integer // pk, fk, unique, check, exclusion
  table_id integer [ref: > table_metadata.id] // 0 if not a table constraint
  index_id integer [ref: > index.id] // The index supporting this constraint, if it's a unique, primary key, foreign key, or exclusion constraint; else 0
  foreign_ref_id integer [ref: > table_metadata.id] // If a foreign key, the referenced table; else 0
  check_src varchar // the expression tree for a check constraint, which provides a textual representation of the constraint expression
} */

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
                    .col(integer(Constraint::TableId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Constraint::Table, Constraint::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Constraint::IndexId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Constraint::Table, Constraint::IndexId)
                            .to(Index::Table, Index::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Constraint::ForeignRefId))
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
