#![allow(dead_code, unused_imports)]

use crate::entities::cascades_group;
use crate::entities::event::Model as event_model;
use crate::entities::logical_expression;
use crate::entities::physical_expression;
use crate::StorageResult;
use sea_orm::prelude::Json;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;
use std::sync::Arc;

/// TODO: documentation
pub enum CatalogSource {
    Iceberg(),
    Mock,
}

/// TODO: documentation
pub enum AttrType {
    Integer,
    Float,
    Varchar,
    Boolean,
}

/// TODO: documentation
pub enum IndexType {
    BTree,
    Hash,
}

/// TODO: documentation
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
}

/// TODO: documentation
pub enum StatType {
    /// `TableRowCount` only applies to table statistics.
    TableRowCount,
    NotNullCount,
    Cardinality,
    Min,
    Max,
}

/// TODO: documentation
#[derive(PartialEq)]
pub enum EpochOption {
    // TODO(lanlou): Could I make i32 -> EpochId?
    Existed(i32),
    New(String, String),
}

/// TODO: documentation
#[derive(Clone)]
pub struct Stat {
    pub stat_type: i32,
    pub stat_value: Json,
    pub attr_ids: Vec<i32>,
    pub table_id: Option<i32>,
    pub name: String,
}

/// TODO: documentation
#[derive(Clone, Debug, PartialEq)]
pub struct Cost {
    pub compute_cost: i32,
    pub io_cost: i32,
    // Raw estimated output row count of targeted expression.
    pub estimated_statistic: i32,
}

/// TODO: documentation
#[trait_variant::make(Send)]
pub trait CostModelStorageLayer {
    type GroupId;
    type TableId;
    type AttrId;
    type ExprId;
    type EpochId;
    type StatId;

    // TODO: Change EpochId to event::Model::epoch_id
    async fn create_new_epoch(&self, source: String, data: String) -> StorageResult<Self::EpochId>;

    async fn update_stats_from_catalog(&self, c: CatalogSource) -> StorageResult<Self::EpochId>;

    async fn update_stats(
        &self,
        stat: Stat,
        epoch_option: EpochOption,
    ) -> StorageResult<Option<Self::EpochId>>;

    async fn store_cost(
        &self,
        expr_id: Self::ExprId,
        cost: Cost,
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
    ) -> StorageResult<Option<Json>>;

    /// Get the (joint) statistics for one or more attributes.
    ///
    /// If `epoch_id` is None, it will return the latest statistics.
    async fn get_stats_for_attr(
        &self,
        attr_ids: Vec<Self::AttrId>,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<Json>>;

    async fn get_cost_analysis(
        &self,
        expr_id: Self::ExprId,
        epoch_id: Self::EpochId,
    ) -> StorageResult<Option<Cost>>;

    async fn get_cost(&self, expr_id: Self::ExprId) -> StorageResult<Option<Cost>>;
}
