use sea_orm_migration::prelude::*;

mod m20241029_000001_cascades_group;
mod m20241029_000001_logical_expression;
mod m20241029_000001_logical_group_junction;
mod m20241029_000001_logical_property;
mod m20241029_000001_physical_expression;
mod m20241029_000001_physical_group_junction;
mod m20241029_000001_physical_property;

use m20241029_000001_cascades_group as cascades_group;
use m20241029_000001_logical_expression as logical_expression;
use m20241029_000001_logical_group_junction as logical_group_junction;
use m20241029_000001_logical_property as logical_property;
use m20241029_000001_physical_expression as physical_expression;
use m20241029_000001_physical_group_junction as physical_group_junction;
use m20241029_000001_physical_property as physical_property;

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
