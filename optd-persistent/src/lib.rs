use sea_orm::*;
use sea_orm_migration::prelude::*;

mod migrator;
use migrator::Migrator;

pub const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    Migrator::refresh(db).await.unwrap();

    assert!(schema_manager.has_table("cascades_group").await?);
    assert!(schema_manager.has_table("logical_expression").await?);
    assert!(schema_manager.has_table("logical_group_junction").await?);
    assert!(schema_manager.has_table("logical_property").await?);
    assert!(schema_manager.has_table("physical_expression").await?);
    assert!(schema_manager.has_table("physical_property").await?);
    assert!(schema_manager.has_table("physical_group_junction").await?);

    Ok(())
}
