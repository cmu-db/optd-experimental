#![allow(dead_code, unused_imports)]

use crate::entities::cascades_group;
use crate::entities::event::Model as event_model;
use crate::entities::logical_expression;
use crate::entities::physical_expression;
use crate::StorageResult;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;
use std::sync::Arc;

pub enum CatalogSource {
    Iceberg(),
}

#[trait_variant::make(Send)]
pub trait CostModelStorageLayer {
    type GroupId;
    type TableId;
    type AttrId;
    type ExprId;
    type EpochId;
    type StatId;

    // TODO: Change EpochId to event::Model::epoch_id
    async fn create_new_epoch(
        &mut self,
        source: String,
        data: String,
    ) -> StorageResult<Self::EpochId>;

    async fn update_stats_from_catalog(
        &self,
        c: CatalogSource,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()>;

    // i32 in `stats:i32` is a placeholder for the stats type
    async fn update_stats(&self, stats: i32, epoch_id: Self::EpochId) -> StorageResult<()>;

    async fn store_cost(
        &self,
        expr_id: Self::ExprId,
        cost: i32,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()>;

    async fn store_expr_stats_mappings(
        &self,
        expr_id: Self::ExprId,
        stat_ids: Vec<Self::StatId>,
    ) -> StorageResult<()>;

    /// Get the statistics for a given table.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_table(
        &self,
        table_id: Self::TableId,
        // TODO: Add enum for stat_type
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<f32>>;

    /// Get the statistics for a given attribute.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_attr(
        &self,
        attr_id: Self::AttrId,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<f32>>;

    /// Get the joint statistics for a list of attributes.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_attrs(
        &self,
        attr_ids: Vec<Self::AttrId>,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<f32>>;

    async fn get_cost_analysis(
        &self,
        expr_id: Self::ExprId,
        epoch_id: Self::EpochId,
    ) -> StorageResult<Option<i32>>;

    async fn get_cost(&self, expr_id: Self::ExprId) -> StorageResult<Option<i32>>;
}
