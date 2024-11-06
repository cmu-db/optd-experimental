#![allow(dead_code, unused_imports)]

use crate::entities::cascades_group;
use crate::entities::event::Model as event_model;
use crate::entities::logical_expression;
use crate::entities::physical_expression;
use crate::{EpochId, ExprId, StatId, StorageResult};
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;
use std::sync::Arc;

pub enum CatalogSource {
    Iceberg(),
}

pub trait CostModelStorageLayer {
    // TODO: Change EpochId to event::Model::epoch_id
    async fn create_new_epoch(&mut self, source: String, data: String) -> StorageResult<EpochId>;

    async fn update_stats_from_catalog(
        &self,
        c: CatalogSource,
        epoch_id: EpochId,
    ) -> StorageResult<()>;

    // i32 in `stats:i32` is a placeholder for the stats type
    async fn update_stats(&self, stats: i32, epoch_id: EpochId) -> StorageResult<()>;

    async fn store_cost(&self, expr_id: ExprId, cost: i32, epoch_id: EpochId) -> StorageResult<()>;

    async fn store_expr_stats_mappings(
        &self,
        expr_id: ExprId,
        stat_ids: Vec<StatId>,
    ) -> StorageResult<()>;

    /// Get the statistics for a given table.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_table(
        &self,
        table_id: i32,
        stat_type: i32,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<f32>>;

    /// Get the statistics for a given attribute.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_attr(
        &self,
        attr_id: i32,
        stat_type: i32,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<f32>>;

    /// Get the joint statistics for a list of attributes.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_attrs(
        &self,
        attr_ids: Vec<i32>,
        stat_type: i32,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<f32>>;

    async fn get_cost_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: EpochId,
    ) -> StorageResult<Option<i32>>;

    async fn get_cost(&self, expr_id: ExprId) -> StorageResult<Option<i32>>;
}
