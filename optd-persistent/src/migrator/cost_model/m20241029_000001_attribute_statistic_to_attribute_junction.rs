/*
Table attribute_stats_junction {
  attr_id integer [ref: > table_attribute.id]
  stats_id integer [ref: > attribute_stats.id]
}
 */

use crate::migrator::catalog::attribute::Attribute;
use crate::migrator::cost_model::attribute_statistic::AttributeStatistic;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum AttributeStatisticToAttributeJunction {
    Table,
    AttributeStatisticId,
    AttributeId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AttributeStatisticToAttributeJunction::Table)
                    .if_not_exists()
                    .col(integer(
                        AttributeStatisticToAttributeJunction::AttributeStatisticId,
                    ))
                    .col(integer(AttributeStatisticToAttributeJunction::AttributeId))
                    .primary_key(
                        Index::create()
                            .col(AttributeStatisticToAttributeJunction::AttributeStatisticId)
                            .col(AttributeStatisticToAttributeJunction::AttributeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeStatisticToAttributeJunction::Table,
                                AttributeStatisticToAttributeJunction::AttributeStatisticId,
                            )
                            .to(AttributeStatistic::Table, AttributeStatistic::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AttributeStatisticToAttributeJunction::Table,
                                AttributeStatisticToAttributeJunction::AttributeId,
                            )
                            .to(Attribute::Table, Attribute::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AttributeStatisticToAttributeJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
