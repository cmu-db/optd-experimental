#![allow(dead_code, unused_imports, unused_variables)]

use crate::{BackendManager, CostModelStorageLayer, CostModelStorageResult};

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
    ) -> CostModelStorageResult<Self::EpochId> {
        todo!()
    }

    async fn update_stats_from_catalog(
        &self,
        c: CatalogSource,
        epoch_id: Self::EpochId,
    ) -> CostModelStorageResult<()> {
        todo!()
    }

    async fn update_stats(
        &self,
        stats: i32,
        epoch_id: Self::EpochId,
    ) -> CostModelStorageResult<()> {
        todo!()
    }

    async fn store_cost(
        &self,
        expr_id: Self::ExprId,
        cost: i32,
        epoch_id: Self::EpochId,
    ) -> CostModelStorageResult<()> {
        todo!()
    }

    async fn store_expr_stats_mappings(
        &self,
        expr_id: Self::ExprId,
        stat_ids: Vec<Self::StatId>,
    ) -> CostModelStorageResult<()> {
        todo!()
    }

    #[doc = " Get the statistics for a given table."]
    #[doc = " If `epoch_id` is None, it will return the latest statistics."]
    async fn get_stats_for_table(
        &self,
        table_id: i32,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> CostModelStorageResult<Option<f32>> {
        todo!()
    }

    #[doc = " Get the statistics for a given attribute."]
    #[doc = " If `epoch_id` is None, it will return the latest statistics."]
    async fn get_stats_for_attr(
        &self,
        attr_id: i32,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> CostModelStorageResult<Option<f32>> {
        todo!()
    }

    #[doc = " Get the joint statistics for a list of attributes."]
    #[doc = " If `epoch_id` is None, it will return the latest statistics."]
    async fn get_stats_for_attrs(
        &self,
        attr_ids: Vec<i32>,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> CostModelStorageResult<Option<f32>> {
        todo!()
    }

    async fn get_cost_analysis(
        &self,
        expr_id: Self::ExprId,
        epoch_id: Self::EpochId,
    ) -> CostModelStorageResult<Option<i32>> {
        todo!()
    }

    async fn get_cost(&self, expr_id: Self::ExprId) -> CostModelStorageResult<Option<i32>> {
        todo!()
    }
}
