//! Very basic demo of using the ORM for optd-persistent.
//!
//! You may run into errors when you first clone this repository.
//! See the `README.md` for setup instructions.

#![allow(dead_code, unused_imports)]

use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;

mod entities;
mod migrator;

use entities::{prelude::*, *};
use optd_persistent::DATABASE_URL;

#[tokio::main]
async fn main() {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    // Create a new `CascadesGroup`.
    let group = cascades_group::ActiveModel {
        winner: ActiveValue::Set(None),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    // Create a new logical expression.
    let l_expr = logical_expression::ActiveModel {
        fingerprint: ActiveValue::Set(42), // Example fingerprint
        data: ActiveValue::Set(json!({ // Example operator
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
        let group = CascadesGroup::find_by_id(*group.id.try_as_ref().unwrap())
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

    // Clean up everything. Since everything is cascading, we only need to manually delete the group
    // and then SeaORM will take care of the expression and the junction.
    group.delete(&db).await.unwrap();

    println!("Demo Finished!");
}
