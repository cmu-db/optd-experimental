/*
Table predicate {
  id integer [pk]
  data json
  variant integer
}
*/

use sea_orm_migration::{
    prelude::*,
    schema::{integer, json, pk_auto},
};

#[derive(Iden)]
pub enum Predicate {
    Table,
    Id,
    Data,
    Variant,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Predicate::Table)
                    .if_not_exists()
                    .col(pk_auto(Predicate::Id))
                    .col(json(Predicate::Data))
                    .col(integer(Predicate::Variant))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Predicate::Table).to_owned())
            .await
    }
}
