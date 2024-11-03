#![allow(dead_code, unused_imports, unused_variables)]

use sea_orm::DatabaseConnection;
use crate::memo_table::{self, EpochId, MemoTable};
use crate::entities::physical_expression;

pub struct ORMManager {
    db_conn: DatabaseConnection,
    // TODO: Change EpochId to event::Model::epoch_id
    latest_epoch_id: EpochId,
}

impl MemoTable for ORMManager {
    async fn create_new_epoch(&self) -> memo_table::EpochId {
        todo!()
    }

    async fn update_stats_from_catalog(&self, c:memo_table::CatalogSource, epoch_id:memo_table::EpochId) -> Result<(), ()> {
        todo!()
    }

    async fn update_stats(&self, stats:i32, epoch_id:memo_table::EpochId) -> Result<(), ()> {
        todo!()
    }

    async fn store_cost(&self, expr_id:memo_table::ExprId, cost:i32, epoch_id:memo_table::EpochId) -> Result<(), ()> {
        todo!()
    }

    async fn get_stats_analysis(&self, table_id:i32, attr_id:Option<i32>, epoch_id:memo_table::EpochId) -> Option<i32> {
        todo!()
    }

    async fn get_stats(&self, table_id:i32, attr_id:Option<i32>) -> Option<i32> {
        todo!()
    }

    async fn get_cost_analysis(&self, expr_id:memo_table::ExprId, epoch_id:memo_table::EpochId) -> Option<i32> {
        todo!()
    }

    async fn get_cost(&self, expr_id:memo_table::ExprId) -> Option<i32> {
        todo!()
    }

    async fn get_group_winner_from_group_id(&self, group_id:i32) -> Option<physical_expression::ActiveModel> {
        todo!()
    }

    async fn add_new_expr(&mut self, expr:memo_table::Expression) -> (memo_table::GroupId, memo_table::ExprId) {
        todo!()
    }

    async fn add_expr_to_group(&mut self, expr: memo_table::Expression, group_id: memo_table::GroupId) -> Option<memo_table::ExprId> {
        todo!()
    }

    async fn get_group_id(&self, expr_id: memo_table::ExprId) -> memo_table::GroupId {
        todo!()
    }

    async fn get_expr_memoed(&self, expr_id: memo_table::ExprId) -> memo_table::Expression {
        todo!()
    }

    async fn get_all_group_ids(&self) -> Vec<memo_table::GroupId> {
        todo!()
    }

    async fn get_group(&self, group_id: memo_table::GroupId) -> crate::entities::cascades_group::ActiveModel {
        todo!()
    }

    async fn update_group_winner(&mut self, group_id: memo_table::GroupId, latest_winner:Option<memo_table::ExprId>) {
        todo!()
    }

    async fn get_all_exprs_in_group(&self, group_id: memo_table::GroupId) -> Vec<memo_table::ExprId> {
        todo!()
    }

    async fn get_group_info(&self, group_id: memo_table::GroupId) -> &Option<memo_table::ExprId> {
        todo!()
    }

    async fn get_predicate_binding(&self, group_id: memo_table::GroupId) -> Option<memo_table::Expression> {
        todo!()
    }

    async fn try_get_predicate_binding(&self, group_id: memo_table::GroupId) -> Option<memo_table::Expression> {
        todo!()
    }
}