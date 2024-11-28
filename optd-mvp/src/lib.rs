use sea_orm::*;
use sea_orm_migration::prelude::*;
use thiserror::Error;

mod migrator;
use migrator::Migrator;

mod entities;

mod memo;
use memo::MemoError;

mod expression;

/// The filename of the SQLite database for migration.
pub const DATABASE_FILENAME: &str = "sqlite.db";
/// The URL of the SQLite database for migration.
pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

/// An error type wrapping all the different kinds of error the optimizer might raise.
///
/// TODO more docs.
#[derive(Error, Debug)]
pub enum OptimizerError {
    #[error("SeaORM error")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("Memo table logical error")]
    Memo(#[from] MemoError),
    #[error("unknown error")]
    Unknown,
}

/// Shorthand for a [`Result`] with an error type [`OptimizerError`].
pub type OptimizerResult<T> = Result<T, OptimizerError>;

/// Applies all migrations.
pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}

/// Helper function for hashing expression data.
///
/// TODO remove this.
fn hash_expression(kind: i16, data: &serde_json::Value) -> i64 {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    kind.hash(&mut hasher);
    data.hash(&mut hasher);
    hasher.finish() as i64
}
