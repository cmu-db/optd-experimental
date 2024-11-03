#![allow(dead_code)]

use std::sync::atomic::AtomicUsize;

use sea_orm::*;
use sea_orm_migration::prelude::*;

use migrator::Migrator;

mod entities;
mod migrator;

mod cost_model;
pub use cost_model::interface::CostModelStorageLayer;

pub type CostModelStorageResult<T> = Result<T, CostModelError>;

pub enum CostModelError {
    // TODO: Add more error types
    Database(DbErr),
}

impl From<DbErr> for CostModelError {
    fn from(value: DbErr) -> Self {
        CostModelError::Database(value)
    }
}

pub struct BackendManager {
    db: DatabaseConnection,
    latest_epoch_id: AtomicUsize,
}

impl BackendManager {
    /// Creates a new `BackendManager`.
    pub async fn new() -> CostModelStorageResult<Self> {
        Ok(Self {
            db: Database::connect(DATABASE_URL).await?,
            latest_epoch_id: AtomicUsize::new(0),
        })
    }
}

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub const DATABASE_FILE: &str = "./sqlite.db";
pub const TEST_DATABASE_URL: &str = "sqlite:./test.db?mode=rwc";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
