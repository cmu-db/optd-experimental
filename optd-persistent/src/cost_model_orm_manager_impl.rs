#![allow(dead_code, unused_imports, unused_variables)]

use crate::cost_model_storage_layer::CostModelStorageLayer;
use crate::orm_manager::ORMManager;

impl CostModelStorageLayer for ORMManager {
    async fn create_new_epoch(
        &mut self,
        source: String,
        data: String,
    ) -> crate::StorageResult<crate::EpochId> {
        todo!()
    }

    async fn update_stats_from_catalog(
        &self,
        c: crate::cost_model_storage_layer::CatalogSource,
        epoch_id: crate::EpochId,
    ) -> crate::StorageResult<()> {
        todo!()
    }

    async fn update_stats(&self, stats: i32, epoch_id: crate::EpochId) -> crate::StorageResult<()> {
        todo!()
    }

    async fn store_cost(
        &self,
        expr_id: crate::ExprId,
        cost: i32,
        epoch_id: crate::EpochId,
    ) -> crate::StorageResult<()> {
        todo!()
    }

    async fn store_expr_stats_mappings(
        &self,
        expr_id: crate::ExprId,
        stat_ids: Vec<crate::StatId>,
    ) -> crate::StorageResult<()> {
        todo!()
    }

    async fn get_stats_for_table(
        &self,
        table_id: i32,
        stat_type: i32,
        epoch_id: Option<crate::EpochId>,
    ) -> crate::StorageResult<Option<f32>> {
        todo!()
    }

    async fn get_stats_for_attr(
        &self,
        attr_id: i32,
        stat_type: i32,
        epoch_id: Option<crate::EpochId>,
    ) -> crate::StorageResult<Option<f32>> {
        todo!()
    }

    async fn get_stats_for_attrs(
        &self,
        attr_ids: Vec<i32>,
        stat_type: i32,
        epoch_id: Option<crate::EpochId>,
    ) -> crate::StorageResult<Option<f32>> {
        todo!()
    }

    async fn get_cost_analysis(
        &self,
        expr_id: crate::ExprId,
        epoch_id: crate::EpochId,
    ) -> crate::StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_cost(&self, expr_id: crate::ExprId) -> crate::StorageResult<Option<i32>> {
        todo!()
    }
}
