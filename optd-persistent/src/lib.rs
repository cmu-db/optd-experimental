#![allow(dead_code)]

use std::sync::LazyLock;

use sea_orm::*;
use sea_orm_migration::prelude::*;

use migrator::Migrator;

pub mod entities;
mod migrator;

pub mod cost_model;
pub use cost_model::interface::CostModelStorageLayer;

pub const DATABASE_FILENAME: &str = "sqlite.db";
pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub const TEST_DATABASE_FILENAME: &str = "init.db";
pub static TEST_DATABASE_FILE: LazyLock<String> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .join("src")
        .join("db")
        .join(TEST_DATABASE_FILENAME)
        .to_str()
        .unwrap()
        .to_owned()
});
pub static TEST_DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| get_sqlite_url(TEST_DATABASE_FILE.as_str()));

fn get_sqlite_url(file: &str) -> String {
    format!("sqlite:{}?mode=rwc", file)
}

pub type StorageResult<T> = Result<T, BackendError>;

#[derive(Debug)]
pub enum CostModelError {
    // TODO: Add more error types
    UnknownStatisticType,
    VersionedStatisticNotFound,
}

#[derive(Debug)]
pub enum BackendError {
    CostModel(CostModelError),
    Database(DbErr),
    // TODO: Add other variants as needed for different error types
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
}

impl BackendManager {
    /// Creates a new `BackendManager`.
    pub async fn new(database_url: Option<&str>) -> StorageResult<Self> {
        Ok(Self {
            db: Database::connect(database_url.unwrap_or(DATABASE_URL)).await?,
        })
    }
}

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
