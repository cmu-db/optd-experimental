use sea_orm::*;
use sea_orm_migration::prelude::*;

mod migrator;
use migrator::Migrator;

mod entities;

/// The filename of the SQLite database for migration.
pub const DATABASE_FILENAME: &str = "sqlite.db";
/// The URL of the SQLite database for migration.
pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
