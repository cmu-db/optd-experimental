use sea_orm::*;
use sea_orm_migration::prelude::*;

mod migrator;
use migrator::Migrator;

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
