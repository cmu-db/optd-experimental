//! This module defines the tables and their schemas for representing a persistent memo table.
//!
//! The most important tables represented here are the [`group`], [`logical_expression`], and
//! [`physical_expression`] tables. See the corresponding modules for more information on their
//! relations and fields.
//!
//! See the SeaORM docs for more information specific to migrations.
//!
//! [`group`]: memo::group
//! [`logical_expression`]: memo::logical_expression
//! [`physical_expression`]: memo::physical_expression

use sea_orm_migration::prelude::*;

mod memo;

/// A unit struct that implements the [`MigratorTrait`] for running custom migrations.
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
