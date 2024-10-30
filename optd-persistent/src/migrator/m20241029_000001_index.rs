/*
Table index {
  id integer PK
  name varchar
  table_id integer [ref: > table_metadata.id]
  number_of_attributes integer
  is_unique boolean
  nulls_not_distinct boolean // Only valid for unique index, if true, then null value is equal, if false, null value is distinct
  is_primary boolean
  is_clustered boolean // If true, the table was last clustered on this index
  is_exclusion boolean // More fields might be added in the future for expressiveness on exclusion constraint.
  data json // Stores the attribute ids. The reason for not creating an additional junction table is the same as with the attribute_stats table.
}
 */

use super::table_metadata::TableMetadata;
use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Index {
    Table,
    Id,
    TableId,
    Name,
    NumberOfAttributes,
    IsUnique,
    NullsNotDistinct,
    IsPrimary,
    IsClustered,
    IsExclusion,
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
                    .table(Index::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Index::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Index::TableId).integer())
                    .col(ColumnDef::new(Index::Name).string())
                    .col(ColumnDef::new(Index::NumberOfAttributes).integer())
                    .col(ColumnDef::new(Index::IsUnique).boolean())
                    .col(ColumnDef::new(Index::NullsNotDistinct).boolean())
                    .col(ColumnDef::new(Index::IsPrimary).boolean())
                    .col(ColumnDef::new(Index::IsClustered).boolean())
                    .col(ColumnDef::new(Index::IsExclusion).boolean())
                    .col(ColumnDef::new(Index::Data).json())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Index::Table, Index::TableId)
                            .to(TableMetadata::Table, TableMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
