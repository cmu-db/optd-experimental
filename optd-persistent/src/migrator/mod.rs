use sea_orm_migration::prelude::*;

mod catalog;
mod cost_model;
mod memo;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(catalog::database_metadata::Migration),
            Box::new(catalog::namespace_metadata::Migration),
            Box::new(catalog::table_metadata::Migration),
            Box::new(catalog::attribute::Migration),
            Box::new(catalog::attribute_constraint_junction::Migration),
            Box::new(catalog::attribute_foreign_constraint_junction::Migration),
            Box::new(catalog::index::Migration),
            Box::new(catalog::trigger::Migration),
            Box::new(catalog::constraint::Migration),
            Box::new(cost_model::statistic::Migration),
            Box::new(cost_model::statistic_to_attribute_junction::Migration),
            Box::new(cost_model::event::Migration),
            Box::new(cost_model::plan_cost::Migration),
            Box::new(memo::cascades_group::Migration),
            Box::new(memo::group_winner::Migration),
            Box::new(memo::logical_expression::Migration),
            Box::new(memo::logical_children::Migration),
            Box::new(memo::logical_property::Migration),
            Box::new(memo::physical_expression::Migration),
            Box::new(memo::physical_children::Migration),
            Box::new(memo::physical_property::Migration),
        ]
    }
}
