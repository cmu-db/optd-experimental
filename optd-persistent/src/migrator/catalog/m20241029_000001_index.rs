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

use crate::migrator::catalog::table_metadata::TableMetadata;
use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(boolean(Index::IsUnique))
                    .col(boolean(Index::NullsNotDistinct))
                    .col(boolean(Index::IsPrimary))
                    .col(boolean(Index::IsClustered))
                    .col(boolean(Index::IsExclusion))
                    .col(json(Index::Data))
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
