use sea_orm_migration::prelude::*;

mod memo;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(memo::group::Migration),
            Box::new(memo::fingerprint::Migration),
            Box::new(memo::logical_expression::Migration),
            Box::new(memo::logical_children::Migration),
            Box::new(memo::physical_expression::Migration),
            Box::new(memo::physical_children::Migration),
        ]
    }
}
