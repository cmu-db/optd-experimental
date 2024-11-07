//! An entity representing the relationship between [`statistic`] and [`attribute`].
//!
//! One [`statistic`] can be associated with multiple [`attribute`]s, which denotes a joint
//! statistic for the attributes. On the other hand, one [`attribute`] can be associated with
//! multiple [`statistic`]s, since the attribute can be used in multiple statistics.

use crate::migrator::catalog::attribute::Attribute;
use crate::migrator::cost_model::statistic::Statistic;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
pub enum StatisticToAttributeJunction {
    Table,
    StatisticId,
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
                    .table(StatisticToAttributeJunction::Table)
                    .if_not_exists()
                    .col(integer(StatisticToAttributeJunction::StatisticId))
                    .col(integer(StatisticToAttributeJunction::AttributeId))
                    .primary_key(
                        Index::create()
                            .col(StatisticToAttributeJunction::StatisticId)
                            .col(StatisticToAttributeJunction::AttributeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                StatisticToAttributeJunction::Table,
                                StatisticToAttributeJunction::StatisticId,
                            )
                            .to(Statistic::Table, Statistic::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                StatisticToAttributeJunction::Table,
                                StatisticToAttributeJunction::AttributeId,
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
                    .table(StatisticToAttributeJunction::Table)
                    .to_owned(),
            )
            .await
    }
}
