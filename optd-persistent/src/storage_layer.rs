#![allow(dead_code, unused_imports)]

use crate::entities::cascades_group;
use crate::entities::event::Model as event_model;
use crate::entities::logical_expression;
use crate::entities::physical_expression;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;
use std::sync::Arc;

pub type GroupId = i32;
pub type ExprId = i32;
pub type EpochId = i32;

pub type StorageResult<T> = Result<T, DbErr>;

pub enum CatalogSource {
    Iceberg(),
}

pub enum Expression {
    LogicalExpression(logical_expression::Model),
    PhysicalExpression(physical_expression::Model),
}

// TODO
// A dummy WinnerInfo struct
// pub struct WinnerInfo {
//     pub expr_id: ExprId,
//     pub total_weighted_cost: f64,
//     pub operation_weighted_cost: f64,
//     pub total_cost: Cost,
//     pub operation_cost: Cost,
//     pub statistics: Arc<Statistics>,
// }
// The optd WinnerInfo struct makes everything too coupled.
pub struct WinnerInfo {}

pub trait StorageLayer {
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

    async fn get_group_winner_from_group_id(
        &self,
        group_id: i32,
    ) -> StorageResult<Option<physical_expression::ActiveModel>>;

    /// Add an expression to the memo table. If the expression already exists, it will return the existing group id and
    /// expr id. Otherwise, a new group and expr will be created.
    async fn add_new_expr(&mut self, expr: Expression) -> StorageResult<(GroupId, ExprId)>;

    /// Add a new expression to an existing group. If the expression is a group, it will merge the two groups. Otherwise,
    /// it will add the expression to the group. Returns the expr id if the expression is not a group.
    async fn add_expr_to_group(
        &mut self,
        expr: Expression,
        group_id: GroupId,
    ) -> StorageResult<Option<ExprId>>;

    /// Get the group id of an expression.
    /// The group id is volatile, depending on whether the groups are merged.
    async fn get_group_id(&self, expr_id: ExprId) -> StorageResult<GroupId>;

    /// Get the memoized representation of a node.
    async fn get_expr_memoed(&self, expr_id: ExprId) -> StorageResult<Expression>;

    /// Get all groups IDs in the memo table.
    async fn get_all_group_ids(&self) -> StorageResult<Vec<GroupId>>;

    /// Get a group by ID
    async fn get_group(&self, group_id: GroupId) -> StorageResult<cascades_group::ActiveModel>;

    /// Update the group winner.
    async fn update_group_winner(
        &mut self,
        group_id: GroupId,
        latest_winner: Option<ExprId>,
    ) -> StorageResult<()>;

    // The below functions can be overwritten by the memo table implementation if there
    // are more efficient way to retrieve the information.

    /// Get all expressions in the group.
    async fn get_all_exprs_in_group(&self, group_id: GroupId) -> StorageResult<Vec<ExprId>>;

    /// Get winner info for a group id
    async fn get_group_info(&self, group_id: GroupId) -> StorageResult<&Option<ExprId>>;

    // TODO:
    /// Get the best group binding based on the cost
    // fn get_best_group_binding(
    //     &self,
    //     group_id: GroupId,
    //     mut post_process: impl FnMut(Arc<Expression>, GroupId, &WinnerInfo),
    // ) -> Result<Expression>;
    // {
    //     // let info: &GroupInfo = this.get_group_info(group_id);
    //     // if let Winner::Full(info @ WinnerInfo { expr_id, .. }) = &info.winner {
    //     //     let expr = this.get_expr_memoed(*expr_id);
    //     //     let mut children = Vec::with_capacity(expr.children.len());
    //     //     for child in &expr.children {
    //     //         children.push(
    //     //             get_best_group_binding_inner(this, *child, post_process)
    //     //                 .with_context(|| format!("when processing expr {}", expr_id))?,
    //     //         );
    //     //     }
    //     //     let node = Arc::new(RelNode {
    //     //         typ: expr.typ.clone(),
    //     //         children,
    //     //         data: expr.data.clone(),
    //     //     });
    //     //     post_process(node.clone(), group_id, info);
    //     //     return Ok(node);
    //     // }
    //     // bail!("no best group binding for group {}", group_id)
    // };

    /// Get all bindings of a predicate group. Will panic if the group contains more than one bindings.
    async fn get_predicate_binding(&self, group_id: GroupId) -> StorageResult<Option<Expression>>;

    /// Get all bindings of a predicate group. Returns None if the group contains zero or more than one bindings.
    async fn try_get_predicate_binding(
        &self,
        group_id: GroupId,
    ) -> StorageResult<Option<Expression>>;
}
