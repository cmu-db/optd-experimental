#![allow(dead_code, unused_imports)]

use crate::entities::cascades_group;
use crate::entities::event::Model as event_model;
use crate::entities::logical_expression;
use crate::entities::physical_expression;
use crate::{ExprId, GroupId, StorageResult};
use sea_orm::*;
use sea_orm_migration::prelude::*;
use serde_json::json;
use std::sync::Arc;

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

pub trait MemoStorageLayer {
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