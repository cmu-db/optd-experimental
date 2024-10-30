use sea_orm_migration::prelude::*;

mod m20241029_000001_attribute_stat;
mod m20241029_000001_attribute_stats_junction;
mod m20241029_000001_cascades_group;
mod m20241029_000001_constraint;
mod m20241029_000001_constraint_attribute_junction;
mod m20241029_000001_cost;
mod m20241029_000001_database_metadata;
mod m20241029_000001_event;
mod m20241029_000001_foreign_constraint_ref_attribute_junction;
mod m20241029_000001_group_winner;
mod m20241029_000001_index;
mod m20241029_000001_logical_expression;
mod m20241029_000001_logical_group_junction;
mod m20241029_000001_logical_property;
mod m20241029_000001_namespace_metadata;
mod m20241029_000001_physical_expression;
mod m20241029_000001_physical_group_junction;
mod m20241029_000001_physical_property;
mod m20241029_000001_table_attribute;
mod m20241029_000001_table_metadata;
mod m20241029_000001_trigger;

use m20241029_000001_attribute_stat as attribute_stat;
use m20241029_000001_attribute_stats_junction as attribute_stats_junction;
use m20241029_000001_cascades_group as cascades_group;
use m20241029_000001_constraint as constraint;
use m20241029_000001_constraint_attribute_junction as constraint_attribute_junction;
use m20241029_000001_cost as cost;
use m20241029_000001_database_metadata as database_metadata;
use m20241029_000001_event as event;
use m20241029_000001_foreign_constraint_ref_attribute_junction as foreign_constraint_ref_attribute_junction;
use m20241029_000001_group_winner as group_winner;
use m20241029_000001_index as index;
use m20241029_000001_logical_expression as logical_expression;
use m20241029_000001_logical_group_junction as logical_group_junction;
use m20241029_000001_logical_property as logical_property;
use m20241029_000001_namespace_metadata as namespace_metadata;
use m20241029_000001_physical_expression as physical_expression;
use m20241029_000001_physical_group_junction as physical_group_junction;
use m20241029_000001_physical_property as physical_property;
use m20241029_000001_table_attribute as table_attribute;
use m20241029_000001_table_metadata as table_metadata;
use m20241029_000001_trigger as trigger;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(cascades_group::Migration),
            Box::new(logical_expression::Migration),
            Box::new(logical_group_junction::Migration),
            Box::new(logical_property::Migration),
            Box::new(physical_expression::Migration),
            Box::new(physical_group_junction::Migration),
            Box::new(physical_property::Migration),

            Box::new(table_metadata::Migration),
            Box::new(table_attribute::Migration),
            Box::new(index::Migration),
            Box::new(namespace_metadata::Migration),
            Box::new(database_metadata::Migration),
            Box::new(event::Migration),
            Box::new(cost::Migration),
            Box::new(group_winner::Migration),
            Box::new(attribute_stat::Migration),
            Box::new(attribute_stats_junction::Migration),
            Box::new(constraint::Migration),
            Box::new(constraint_attribute_junction::Migration),
            Box::new(trigger::Migration),
            Box::new(foreign_constraint_ref_attribute_junction::Migration),
        ]
    }
}
