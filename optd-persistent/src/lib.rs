use sea_orm::*;
use sea_orm_migration::prelude::*;

mod entities;
mod migrator;
mod orm_manager;
mod storage_layer;
use migrator::Migrator;

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub const DATABASE_FILE: &str = "./sqlite.db";
pub const TEST_DATABASE_URL: &str = "sqlite:./test.db?mode=rwc";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
