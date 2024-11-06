#![allow(dead_code, unused_imports, unused_variables)]

use crate::entities::{prelude::*, *};
use crate::orm_manager::{Event, PlanCost};
use crate::storage_layer::{self, EpochId, StorageLayer, StorageResult};
use crate::DATABASE_URL;
use sea_orm::*;
use sqlx::types::chrono::Utc;

pub struct ORMManager {
    db_conn: DatabaseConnection,
    // TODO: Change EpochId to event::Model::epoch_id
    latest_epoch_id: EpochId,
}

impl ORMManager {
    pub async fn new(database_url: Option<&str>) -> Self {
        let latest_epoch_id = -1;
        let db_conn = Database::connect(database_url.unwrap_or(DATABASE_URL))
            .await
            .unwrap();
        Self {
            db_conn,
            latest_epoch_id,
        }
    }
}

impl StorageLayer for ORMManager {
    async fn create_new_epoch(
        &mut self,
        source: String,
        data: String,
    ) -> StorageResult<storage_layer::EpochId> {
        let new_event = event::ActiveModel {
            source_variant: sea_orm::ActiveValue::Set(source),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(data)),
            ..Default::default()
        };
        let res = Event::insert(new_event).exec(&self.db_conn).await;
        res.and_then(|insert_res| {
            self.latest_epoch_id = insert_res.last_insert_id;
            Ok(self.latest_epoch_id)
        })
    }

    async fn update_stats_from_catalog(
        &self,
        c: storage_layer::CatalogSource,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn update_stats(
        &self,
        stats: i32,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn store_cost(
        &self,
        expr_id: storage_layer::ExprId,
        cost: i32,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<()> {
        // TODO: update PhysicalExpression and Event tables
        // Check if expr_id exists in PhysicalExpression table
        let expr_exists = PhysicalExpression::find_by_id(expr_id)
            .one(&self.db_conn)
            .await?;
        if expr_exists.is_none() {
            return Err(DbErr::RecordNotFound(
                "ExprId not found in PhysicalExpression table".to_string(),
            ));
        }

        // Check if epoch_id exists in Event table
        let epoch_exists = Event::find()
            .filter(event::Column::EpochId.eq(epoch_id))
            .one(&self.db_conn)
            .await
            .unwrap();

        let new_cost = plan_cost::ActiveModel {
            physical_expression_id: ActiveValue::Set(expr_id),
            epoch_id: ActiveValue::Set(epoch_id),
            cost: ActiveValue::Set(cost),
            is_valid: ActiveValue::Set(true),
            ..Default::default()
        };
        PlanCost::insert(new_cost)
            .exec(&self.db_conn)
            .await
            .map(|_| ())
    }

    async fn get_stats_for_table(
        &self,
        table_id: i32,
        stat_type: i32,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_stats_for_attr(
        &self,
        attr_id: i32,
        stat_type: i32,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_stats_for_attrs(
        &self,
        attr_ids: Vec<i32>,
        stat_type: i32,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_cost_analysis(
        &self,
        expr_id: storage_layer::ExprId,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<Option<i32>> {
        let cost = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .filter(plan_cost::Column::EpochId.eq(epoch_id))
            .one(&self.db_conn)
            .await?;
        assert!(cost.is_some(), "Cost not found in Cost table");
        assert!(cost.clone().unwrap().is_valid, "Cost is not valid");
        Ok(cost.map(|c| c.cost))
    }

    /// Get the latest cost for an expression
    async fn get_cost(&self, expr_id: storage_layer::ExprId) -> StorageResult<Option<i32>> {
        let cost = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .order_by_desc(plan_cost::Column::EpochId)
            .one(&self.db_conn)
            .await?;
        assert!(cost.is_some(), "Cost not found in Cost table");
        assert!(cost.clone().unwrap().is_valid, "Cost is not valid");
        Ok(cost.map(|c| c.cost))
    }

    async fn get_group_winner_from_group_id(
        &self,
        group_id: i32,
    ) -> StorageResult<Option<physical_expression::ActiveModel>> {
        todo!()
    }

    async fn add_new_expr(
        &mut self,
        expr: storage_layer::Expression,
    ) -> StorageResult<(storage_layer::GroupId, storage_layer::ExprId)> {
        todo!()
    }

    async fn add_expr_to_group(
        &mut self,
        expr: storage_layer::Expression,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Option<storage_layer::ExprId>> {
        todo!()
    }

    async fn get_group_id(
        &self,
        expr_id: storage_layer::ExprId,
    ) -> StorageResult<storage_layer::GroupId> {
        todo!()
    }

    async fn get_expr_memoed(
        &self,
        expr_id: storage_layer::ExprId,
    ) -> StorageResult<storage_layer::Expression> {
        todo!()
    }

    async fn get_all_group_ids(&self) -> StorageResult<Vec<storage_layer::GroupId>> {
        todo!()
    }

    async fn get_group(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<crate::entities::cascades_group::ActiveModel> {
        todo!()
    }

    async fn update_group_winner(
        &mut self,
        group_id: storage_layer::GroupId,
        latest_winner: Option<storage_layer::ExprId>,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn get_all_exprs_in_group(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Vec<storage_layer::ExprId>> {
        todo!()
    }

    async fn get_group_info(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<&Option<storage_layer::ExprId>> {
        todo!()
    }

    async fn get_predicate_binding(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Option<storage_layer::Expression>> {
        todo!()
    }

    async fn try_get_predicate_binding(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Option<storage_layer::Expression>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::migrate;
    use sea_orm::{ConnectionTrait, Database, EntityTrait, ModelTrait};
    use serde_json::de;

    use crate::entities::event::Entity as Event;
    use crate::storage_layer::StorageLayer;
    use crate::TEST_DATABASE_URL;

    async fn run_migration() {
        let _ = std::fs::remove_file(TEST_DATABASE_URL);

        let db = Database::connect(TEST_DATABASE_URL)
            .await
            .expect("Unable to connect to the database");

        migrate(&db)
            .await
            .expect("Something went wrong during migration");

        db.execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "PRAGMA foreign_keys = ON;".to_owned(),
        ))
        .await
        .expect("Unable to enable foreign keys");
    }

    #[tokio::test]
    async fn test_create_new_epoch() {
        run_migration().await;
        let mut orm_manager = super::ORMManager::new(Some(TEST_DATABASE_URL)).await;
        let res = orm_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await;
        println!("{:?}", res);
        assert!(res.is_ok());
        assert_eq!(
            super::Event::find()
                .all(&orm_manager.db_conn)
                .await
                .unwrap()
                .len(),
            1
        );
        println!(
            "{:?}",
            super::Event::find()
                .all(&orm_manager.db_conn)
                .await
                .unwrap()[0]
        );
        assert_eq!(
            super::Event::find()
                .all(&orm_manager.db_conn)
                .await
                .unwrap()[0]
                .epoch_id,
            res.unwrap()
        );
    }

    #[tokio::test]
    #[ignore] // Need to update all tables
    async fn test_store_cost() {
        run_migration().await;
        let mut orm_manager = super::ORMManager::new(Some(TEST_DATABASE_URL)).await;
        let epoch_id = orm_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await
            .unwrap();
        let expr_id = 1;
        let cost = 42;
        let res = orm_manager.store_cost(expr_id, cost, epoch_id).await;
        match res {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);
            }
        }
        let costs = super::PlanCost::find()
            .all(&orm_manager.db_conn)
            .await
            .unwrap();
        assert_eq!(costs.len(), 1);
        assert_eq!(costs[0].epoch_id, epoch_id);
        assert_eq!(costs[0].physical_expression_id, expr_id);
        assert_eq!(costs[0].cost, cost);
    }
}
