#![allow(dead_code)]

use std::sync::atomic::AtomicUsize;

use sea_orm::*;
use sea_orm_migration::prelude::*;

use migrator::Migrator;

mod entities;
mod migrator;

mod cost_model;
pub use cost_model::interface::CostModelStorageLayer;

pub type StorageResult<T> = Result<T, BackendError>;

pub enum CostModelError {
    // TODO: Add more error types
    UnknownStatisticType,
}

pub enum BackendError {
    CostModel(CostModelError),
    Database(DbErr),
    // Add other variants as needed for different error types
}

impl From<CostModelError> for BackendError {
    fn from(value: CostModelError) -> Self {
        BackendError::CostModel(value)
    }
}

impl From<DbErr> for BackendError {
    fn from(value: DbErr) -> Self {
        BackendError::Database(value)
    }
}

pub struct BackendManager {
    db: DatabaseConnection,
    latest_epoch_id: AtomicUsize,
}

impl BackendManager {
    /// Creates a new `BackendManager`.
    pub async fn new(database_url: Option<&str>) -> StorageResult<Self> {
        Ok(Self {
            db: Database::connect(database_url.unwrap_or(DATABASE_URL)).await?,
            latest_epoch_id: AtomicUsize::new(0),
        })
    }
}

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub const DATABASE_FILE: &str = "./sqlite.db";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
