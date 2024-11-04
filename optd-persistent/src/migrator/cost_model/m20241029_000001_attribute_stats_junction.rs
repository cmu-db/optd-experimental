/*
Table attribute_stats_junction {
  attr_id integer [ref: > table_attribute.id]
  stats_id integer [ref: > attribute_stats.id]
}
 */

use crate::migrator::cost_model::attribute_stat::AttributeStat;
use crate::migrator::cost_model::table_attribute::TableAttribute;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(Iden)]
pub enum AttributeStatsJunction {
    Table,
    AttrId,
    StatsId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AttributeStatsJunction::Table)
                    .if_not_exists()
                    .col(integer(AttributeStatsJunction::AttrId).not_null())
                    .col(integer(AttributeStatsJunction::StatsId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeStatsJunction::Table,
                                AttributeStatsJunction::AttrId,
                            )
                            .to(TableAttribute::Table, TableAttribute::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeStatsJunction::Table,
                                AttributeStatsJunction::StatsId,
                            )
                            .to(AttributeStat::Table, AttributeStat::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(AttributeStatsJunction::AttrId)
                            .col(AttributeStatsJunction::StatsId)
                            .name("attribute_stats_junction_pk")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AttributeStatsJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
