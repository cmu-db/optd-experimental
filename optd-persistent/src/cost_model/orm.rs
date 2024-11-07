#![allow(dead_code, unused_imports, unused_variables)]

use sea_orm::{sqlx::types::chrono::Utc, EntityTrait};

use crate::{
    entities::event::{self, Entity as Event},
    BackendManager, CostModelStorageLayer, StorageResult,
};

use super::interface::CatalogSource;

impl CostModelStorageLayer for BackendManager {
    type GroupId = i32;
    type TableId = i32;
    type AttrId = i32;
    type ExprId = i32;
    type EpochId = i32;
    type StatId = i32;

    async fn create_new_epoch(
        &mut self,
        source: String,
        data: String,
    ) -> StorageResult<Self::EpochId> {
        let new_event = event::ActiveModel {
            source_variant: sea_orm::ActiveValue::Set(source),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(data)),
            ..Default::default()
        };
        let res = Event::insert(new_event).exec(&self.db).await;
        Ok(res.and_then(|insert_res| {
            self.latest_epoch_id.store(
                insert_res.last_insert_id as usize,
                std::sync::atomic::Ordering::Relaxed,
            );
            Ok(insert_res.last_insert_id)
        })?)
    }

    async fn update_stats_from_catalog(
        &self,
        c: CatalogSource,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn update_stats(&self, stats: i32, epoch_id: Self::EpochId) -> StorageResult<()> {
        todo!()
    }

    async fn store_cost(
        &self,
        expr_id: Self::ExprId,
        cost: i32,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn store_expr_stats_mappings(
        &self,
        expr_id: Self::ExprId,
        stat_ids: Vec<Self::StatId>,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn get_stats_for_table(
        &self,
        table_id: i32,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<f32>> {
        todo!()
    }

    async fn get_stats_for_attr(
        &self,
        attr_id: i32,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<f32>> {
        todo!()
    }

    async fn get_stats_for_attrs(
        &self,
        attr_ids: Vec<i32>,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<f32>> {
        todo!()
    }

    async fn get_cost_analysis(
        &self,
        expr_id: Self::ExprId,
        epoch_id: Self::EpochId,
    ) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_cost(&self, expr_id: Self::ExprId) -> StorageResult<Option<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{migrate, CostModelStorageLayer};
    use sea_orm::{ConnectionTrait, Database, EntityTrait, ModelTrait};
    use serde_json::de;

    use crate::entities::event::Entity as Event;

    async fn run_migration(db_file: &str) -> String {
        let database_url = format!("sqlite:./{}?mode=rwc", db_file);
        remove_db_file(db_file);

        let db = Database::connect(database_url.clone())
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
        database_url.clone()
    }

    fn remove_db_file(db_file: &str) {
        let database_file = format!("./{}", db_file);
        let _ = std::fs::remove_file(database_file);
    }

    #[tokio::test]
    async fn test_create_new_epoch() {
        const DATABASE_FILE: &str = "test_create_new_epoch.db";
        let database_url = run_migration(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let res = backend_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await;
        println!("{:?}", res);
        assert!(res.is_ok());
        assert_eq!(
            super::Event::find()
                .all(&backend_manager.db)
                .await
                .unwrap()
                .len(),
            1
        );
        println!(
            "{:?}",
            super::Event::find().all(&backend_manager.db).await.unwrap()[0]
        );
        assert_eq!(
            super::Event::find().all(&backend_manager.db).await.unwrap()[0].epoch_id,
            res.unwrap()
        );
        remove_db_file(DATABASE_FILE);
    }
}
