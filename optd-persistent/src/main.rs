mod entities;
mod migrator;

use migrator::Migrator;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
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

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    println!("{:?}", db.get_database_backend());

    migrate(&db).await.unwrap();

    // Create a new Group.
    let group = cascades_group::ActiveModel {
        winner: ActiveValue::Set(None),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    // Create a new logical expression.
    let l_expr = logical_expression::ActiveModel {
        fingerprint: ActiveValue::Set(42), // Made up fingerprint
        data: ActiveValue::Set(json!({
            "type": "Scan",
            "table": "lineitem",
            "predicate": "l_quantity < 10",
        })),
        group_id: group.id.clone(),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    // Create a link between the group and the logical expression in the junction table.
    let _link = logical_group_junction::ActiveModel {
        group_id: group.id.clone(),
        logical_expression_id: l_expr.id.clone(),
    }
    .insert(&db)
    .await
    .unwrap();

    // Basic lookup test on each table.
    {
        let groups: Vec<cascades_group::Model> = CascadesGroup::find().all(&db).await.unwrap();
        assert_eq!(groups.len(), 1);

        let l_expressions: Vec<logical_expression::Model> =
            LogicalExpression::find().all(&db).await.unwrap();
        assert_eq!(l_expressions.len(), 1);
    }

    // Retrieve all logical expressions that belong to this group with lazy loading.
    {
        let group = CascadesGroup::find_by_id(group.id.unwrap())
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        let group_expressions: Vec<logical_expression::Model> = group
            .find_related(LogicalExpression)
            .all(&db)
            .await
            .unwrap();
        assert_eq!(group_expressions.len(), 1);
    }

    // Retrieve all logical expressions that belong to this group with eager loading.
    {
        let group_with_expressions: Vec<(cascades_group::Model, Vec<logical_expression::Model>)> =
            CascadesGroup::find()
                .find_with_related(LogicalExpression)
                .all(&db)
                .await
                .unwrap();
        assert_eq!(group_with_expressions.len(), 1);
        assert_eq!(group_with_expressions[0].1.len(), 1);
    }

    Ok(())
}
