use sea_orm_migration::prelude::*;

mod cost_model;
mod memo;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(memo::cascades_group::Migration),
            Box::new(memo::group_winner::Migration),
            Box::new(memo::logical_expression::Migration),
            Box::new(memo::logical_group_junction::Migration),
            Box::new(memo::logical_property::Migration),
            Box::new(memo::physical_expression::Migration),
            Box::new(memo::physical_group_junction::Migration),
            Box::new(memo::physical_property::Migration),
            Box::new(cost_model::database_metadata::Migration),
            Box::new(cost_model::namespace_metadata::Migration),
            Box::new(cost_model::table_metadata::Migration),
            Box::new(cost_model::table_attribute::Migration),
            Box::new(cost_model::attribute_stat::Migration),
            Box::new(cost_model::constraint::Migration),
            Box::new(cost_model::attribute_stats_junction::Migration),
            Box::new(cost_model::constraint_attribute_junction::Migration),
            Box::new(cost_model::foreign_constraint_ref_attribute_junction::Migration),
            Box::new(cost_model::index::Migration),
            Box::new(cost_model::event::Migration),
            Box::new(cost_model::cost::Migration),
            Box::new(cost_model::trigger::Migration),
        ]
    }
}
