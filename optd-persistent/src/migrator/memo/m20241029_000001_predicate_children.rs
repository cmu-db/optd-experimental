/*
Table predicate_children {
  parent_id integer [ref: > predicate.id]
  child_id integer [ref: > predicate.id]
}
 */

use sea_orm_migration::{prelude::*, schema::integer};

use super::m20241029_000001_predicate::Predicate;

#[derive(Iden)]
pub enum PredicateChildren {
    Table,
    ParentId,
    ChildId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PredicateChildren::Table)
                    .if_not_exists()
                    .col(integer(PredicateChildren::ParentId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PredicateChildren::Table, PredicateChildren::ParentId)
                            .name("ParentId")
                            .to(Predicate::Table, Predicate::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PredicateChildren::ChildId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PredicateChildren::Table, PredicateChildren::ChildId)
                            .name("ChildId")
                            .to(Predicate::Table, Predicate::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(PredicateChildren::ParentId)
                            .col(PredicateChildren::ChildId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PredicateChildren::Table).to_owned())
            .await
    }
}
