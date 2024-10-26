mod entities;
mod migrator;

use migrator::Migrator;
use sea_orm::*;
use sea_orm_migration::prelude::*;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    Migrator::refresh(db).await?;

    assert!(schema_manager.has_table("cascades_group").await?);
    assert!(schema_manager.has_table("logical_expression").await?);
    assert!(schema_manager.has_table("physical_expression").await?);
    assert!(schema_manager.has_table("logical_property").await?);
    assert!(schema_manager.has_table("physical_property").await?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    println!("{:?}", db.get_database_backend());

    migrate(&db).await?;

    Ok(())
}
