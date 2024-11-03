#![allow(dead_code, unused_imports)]

use std::sync::Arc;
use crate::entities::cascades_group;
use crate::entities::event::Model as event_model;
use crate::entities::physical_expression;
use crate::entities::logical_expression;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;

pub type GroupId = i32;
pub type ExprId = i32;
pub type EpochId = i32;

pub enum CatalogSource {
    Iceberg(),
}

pub enum Expression {
    LogicalExpression(logical_expression::Model),
    PhysicalExpression(physical_expression::Model)
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
pub struct WinnerInfo {
}

pub trait MemoTable {
    // TODO: Change EpochId to event::Model::epoch_id
    async fn create_new_epoch(&self) -> EpochId;
    async fn update_stats_from_catalog(&self, c:CatalogSource, epoch_id:EpochId) -> Result<(), ()>;
    // i32 in `stats:i32` is a placeholder for the stats type
    async fn update_stats(&self, stats:i32, epoch_id:EpochId) -> Result<(), ()>;
    async fn store_cost(&self, expr_id:ExprId, cost:i32, epoch_id:EpochId) -> Result<(), ()>;
    // table_id, attr_id OR expr_id and return a vector?
    async fn get_stats_analysis(&self, table_id:i32, attr_id:Option<i32>, epoch_id:EpochId) -> Option<i32>;
    async fn get_stats(&self, table_id:i32, attr_id:Option<i32>) -> Option<i32>;
    async fn get_cost_analysis(&self, expr_id:ExprId, epoch_id:EpochId) -> Option<i32>;
    async fn get_cost(&self, expr_id:ExprId) -> Option<i32>;

    async fn get_group_winner_from_group_id(&self, group_id:i32) -> Option<physical_expression::ActiveModel>;

    /// Add an expression to the memo table. If the expression already exists, it will return the existing group id and
    /// expr id. Otherwise, a new group and expr will be created.
    async fn add_new_expr(&mut self, expr:Expression) -> (GroupId, ExprId);

    /// Add a new expression to an existing group. If the expression is a group, it will merge the two groups. Otherwise,
    /// it will add the expression to the group. Returns the expr id if the expression is not a group.
    async fn add_expr_to_group(&mut self, expr: Expression, group_id: GroupId) -> Option<ExprId>;

    /// Get the group id of an expression.
    /// The group id is volatile, depending on whether the groups are merged.
    async fn get_group_id(&self, expr_id: ExprId) -> GroupId;

    /// Get the memoized representation of a node.
    async fn get_expr_memoed(&self, expr_id: ExprId) -> Expression;

    /// Get all groups IDs in the memo table.
    async fn get_all_group_ids(&self) -> Vec<GroupId>;

    /// Get a group by ID
    async fn get_group(&self, group_id: GroupId) -> cascades_group::ActiveModel;

    /// Update the group winner.
    async fn update_group_winner(&mut self, group_id: GroupId, latest_winner:Option<ExprId>);

    // The below functions can be overwritten by the memo table implementation if there
    // are more efficient way to retrieve the information.

    /// Get all expressions in the group.
    async fn get_all_exprs_in_group(&self, group_id: GroupId) -> Vec<ExprId>; 

    /// Get winner info for a group id
    async fn get_group_info(&self, group_id: GroupId) -> &Option<ExprId>;

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
    async fn get_predicate_binding(&self, group_id: GroupId) -> Option<Expression>; 

    /// Get all bindings of a predicate group. Returns None if the group contains zero or more than one bindings.
    async fn try_get_predicate_binding(&self, group_id: GroupId) -> Option<Expression>;
    
}
