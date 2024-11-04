#![allow(dead_code, unused_imports, unused_variables)]

use crate::entities::event::Entity as Event;
use crate::entities::{prelude::*, *};
use crate::storage_layer::{self, EpochId, StorageLayer};
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
    ) -> Result<storage_layer::EpochId, ()> {
        let new_event = event::ActiveModel {
            source_variant: sea_orm::ActiveValue::Set(source),
            create_timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(data)),
            ..Default::default()
        };
        let res = Event::insert(new_event).exec(&self.db_conn).await;
        match res {
            Ok(insert_res) => {
                self.latest_epoch_id = insert_res.last_insert_id;
                Ok(self.latest_epoch_id)
            }
            Err(_) => Err(()),
        }
    }

    async fn update_stats_from_catalog(
        &self,
        c: storage_layer::CatalogSource,
        epoch_id: storage_layer::EpochId,
    ) -> Result<(), ()> {
        todo!()
    }

    async fn update_stats(&self, stats: i32, epoch_id: storage_layer::EpochId) -> Result<(), ()> {
        todo!()
    }

    async fn store_cost(
        &self,
        expr_id: storage_layer::ExprId,
        cost: i32,
        epoch_id: storage_layer::EpochId,
    ) -> Result<(), DbErr> {
        let new_cost = cost::ActiveModel {
            expr_id: ActiveValue::Set(expr_id),
            epoch_id: ActiveValue::Set(epoch_id),
            cost: ActiveValue::Set(cost),
            valid: ActiveValue::Set(true),
            ..Default::default()
        };
        let res = Cost::insert(new_cost).exec(&self.db_conn).await;
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(DbErr::RecordNotInserted),
        }
    }

    async fn get_stats_analysis(
        &self,
        table_id: i32,
        attr_id: Option<i32>,
        epoch_id: storage_layer::EpochId,
    ) -> Option<i32> {
        todo!()
    }

    async fn get_stats(&self, table_id: i32, attr_id: Option<i32>) -> Option<i32> {
        todo!()
    }

    async fn get_cost_analysis(
        &self,
        expr_id: storage_layer::ExprId,
        epoch_id: storage_layer::EpochId,
    ) -> Option<i32> {
        todo!()
    }

    async fn get_cost(&self, expr_id: storage_layer::ExprId) -> Option<i32> {
        todo!()
    }

    async fn get_group_winner_from_group_id(
        &self,
        group_id: i32,
    ) -> Option<physical_expression::ActiveModel> {
        todo!()
    }

    async fn add_new_expr(
        &mut self,
        expr: storage_layer::Expression,
    ) -> (storage_layer::GroupId, storage_layer::ExprId) {
        todo!()
    }

    async fn add_expr_to_group(
        &mut self,
        expr: storage_layer::Expression,
        group_id: storage_layer::GroupId,
    ) -> Option<storage_layer::ExprId> {
        todo!()
    }

    async fn get_group_id(&self, expr_id: storage_layer::ExprId) -> storage_layer::GroupId {
        todo!()
    }

    async fn get_expr_memoed(&self, expr_id: storage_layer::ExprId) -> storage_layer::Expression {
        todo!()
    }

    async fn get_all_group_ids(&self) -> Vec<storage_layer::GroupId> {
        todo!()
    }

    async fn get_group(
        &self,
        group_id: storage_layer::GroupId,
    ) -> crate::entities::cascades_group::ActiveModel {
        todo!()
    }

    async fn update_group_winner(
        &mut self,
        group_id: storage_layer::GroupId,
        latest_winner: Option<storage_layer::ExprId>,
    ) {
        todo!()
    }

    async fn get_all_exprs_in_group(
        &self,
        group_id: storage_layer::GroupId,
    ) -> Vec<storage_layer::ExprId> {
        todo!()
    }

    async fn get_group_info(
        &self,
        group_id: storage_layer::GroupId,
    ) -> &Option<storage_layer::ExprId> {
        todo!()
    }

    async fn get_predicate_binding(
        &self,
        group_id: storage_layer::GroupId,
    ) -> Option<storage_layer::Expression> {
        todo!()
    }

    async fn try_get_predicate_binding(
        &self,
        group_id: storage_layer::GroupId,
    ) -> Option<storage_layer::Expression> {
        todo!()
    }
}

// NOTE: Please run `cargo run --bin migrate_test` before you want to run this test.
#[cfg(test)]
mod tests {
    use sea_orm::{EntityTrait, ModelTrait};
    use serde_json::de;

    use crate::entities::event::Entity as Event;
    use crate::storage_layer::StorageLayer;
    use crate::TEST_DATABASE_URL;

    async fn delete_all_events(orm_manager: &mut super::ORMManager) {
        let events = super::Event::find()
            .all(&orm_manager.db_conn)
            .await
            .unwrap();
        for event in events {
            event.delete(&orm_manager.db_conn).await.unwrap();
        }
    }

    async fn delete_all_costs(orm_manager: &mut super::ORMManager) {
        let costs = super::Cost::find().all(&orm_manager.db_conn).await.unwrap();
        for cost in costs {
            cost.delete(&orm_manager.db_conn).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_create_new_epoch() {
        let mut orm_manager = super::ORMManager::new(Some(TEST_DATABASE_URL)).await;
        delete_all_events(&mut orm_manager).await;
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
        delete_all_events(&mut orm_manager).await;
    }

    #[tokio::test]
    async fn test_store_cost() {
        let mut orm_manager = super::ORMManager::new(Some(TEST_DATABASE_URL)).await;
        delete_all_costs(&mut orm_manager).await;
        let epoch_id = orm_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await
            .unwrap();
        let expr_id = 1;
        let cost = 42;
        let res = orm_manager.store_cost(expr_id, cost, epoch_id).await;
        assert!(res.is_ok());
        let costs = super::Cost::find().all(&orm_manager.db_conn).await.unwrap();
        assert_eq!(costs.len(), 1);
        assert_eq!(costs[0].epoch_id, epoch_id);
        assert_eq!(costs[0].expr_id, expr_id);
        assert_eq!(costs[0].cost, cost);
        delete_all_events(&mut orm_manager).await;
    }
}
