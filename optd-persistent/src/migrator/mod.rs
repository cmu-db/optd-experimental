use sea_orm_migration::prelude::*;

mod m20241026_000001_create_cascades_group_table;
mod m20241026_000001_create_logical_expression_table;
mod m20241026_000001_create_logical_group_junction_table;
mod m20241026_000001_create_logical_property_table;
mod m20241026_000001_create_physical_expression_table;
mod m20241026_000001_create_physical_group_junction_table;
mod m20241026_000001_create_physical_property_table;

use m20241026_000001_create_cascades_group_table as cascades_group;
use m20241026_000001_create_logical_expression_table as logical_expression;
use m20241026_000001_create_logical_group_junction_table as logical_group_junction;
use m20241026_000001_create_logical_property_table as logical_property;
use m20241026_000001_create_physical_expression_table as physical_expression;
use m20241026_000001_create_physical_group_junction_table as physical_group_junction;
use m20241026_000001_create_physical_property_table as physical_property;

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
        ]
    }
}
