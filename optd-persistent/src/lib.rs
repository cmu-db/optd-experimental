use sea_orm::*;
use sea_orm_migration::prelude::*;

mod cost_model_orm_manager_impl;
mod cost_model_storage_layer;
mod entities;
mod memo_orm_manager_impl;
mod memo_storage_layer;
mod migrator;
mod orm_manager;

use migrator::Migrator;

pub type GroupId = i32;
pub type ExprId = i32;
pub type EpochId = i32;
pub type StatId = i32;

pub type StorageResult<T> = Result<T, DbErr>;

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub const DATABASE_FILE: &str = "./sqlite.db";
pub const TEST_DATABASE_URL: &str = "sqlite:./test.db?mode=rwc";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
