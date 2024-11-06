#![allow(dead_code)]

use std::sync::LazyLock;

use sea_orm::*;
use sea_orm_migration::prelude::*;

use migrator::Migrator;

pub mod entities;
mod migrator;

pub mod cost_model;
pub use cost_model::interface::CostModelStorageLayer;

mod memo;
pub use memo::interface::Memo;

/// The filename of the SQLite database for migration.
pub const DATABASE_FILENAME: &str = "sqlite.db";
/// The URL of the SQLite database for migration.
pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

/// The filename of the SQLite database for testing.
pub const TEST_DATABASE_FILENAME: &str = "init.db";
/// The URL of the SQLite database for testing.
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
/// The URL of the SQLite database for testing.
pub static TEST_DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| get_sqlite_url(TEST_DATABASE_FILE.as_str()));

fn get_sqlite_url(file: &str) -> String {
    format!("sqlite:{}?mode=rwc", file)
}

#[derive(Debug)]
pub enum CostModelError {
    // TODO: Add more error types
    UnknownStatisticType,
    VersionedStatisticNotFound,
    CustomError(String),
}

/// TODO convert this to `thiserror`
#[derive(Debug)]
/// The different kinds of errors that might occur while running operations on a memo table.
pub enum MemoError {
    UnknownGroup,
    UnknownLogicalExpression,
    UnknownPhysicalExpression,
    InvalidExpression,
}

/// TODO convert this to `thiserror`
#[derive(Debug)]
pub enum BackendError {
    Memo(MemoError),
    DatabaseError(DbErr),
    CostModel(CostModelError),
    BackendError(String),
}

impl From<String> for CostModelError {
    fn from(value: String) -> Self {
        CostModelError::CustomError(value)
    }
}

impl From<CostModelError> for BackendError {
    fn from(value: CostModelError) -> Self {
        BackendError::CostModel(value)
    }
}

impl From<MemoError> for BackendError {
    fn from(value: MemoError) -> Self {
        BackendError::Memo(value)
    }
}

impl From<DbErr> for BackendError {
    fn from(value: DbErr) -> Self {
        BackendError::DatabaseError(value)
    }
}

/// A type alias for a result with [`BackendError`] as the error type.
pub type StorageResult<T> = Result<T, BackendError>;

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
