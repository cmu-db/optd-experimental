use sea_orm_migration::prelude::*;

pub mod m20241026_000001_create_cascades_group_table;
pub mod m20241026_000001_create_logical_expression_table;
pub mod m20241026_000001_create_physical_expression_table;
pub mod m20241026_000001_create_logical_property_table;
pub mod m20241026_000001_create_physical_property_table;

pub use m20241026_000001_create_cascades_group_table as cascades_group;
pub use m20241026_000001_create_logical_expression_table as logical_expression;
pub use m20241026_000001_create_physical_expression_table as physical_expression;
pub use m20241026_000001_create_logical_property_table as logical_property;
pub use m20241026_000001_create_physical_property_table as physical_property;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(cascades_group::Migration),
            Box::new(logical_expression::Migration),
            Box::new(physical_expression::Migration),
            Box::new(logical_property::Migration),
            Box::new(physical_property::Migration),
        ]
    }
}
