use sea_orm::*;
use sea_orm_migration::prelude::*;

mod migrator;
mod orm_manager;
mod storage_layer;
mod entities;
use migrator::Migrator;

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub const DATABASE_FILE: &str = "./sqlite.db";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
