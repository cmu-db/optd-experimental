#![allow(dead_code, unused_imports, unused_variables)]

use crate::entities::physical_expression;
use crate::storage_layer::{self, EpochId, StorageLayer, StorageResult};
use sea_orm::DatabaseConnection;

pub struct ORMManager {
    db_conn: DatabaseConnection,
    // TODO: Change EpochId to event::Model::epoch_id
    latest_epoch_id: EpochId,
}

impl StorageLayer for ORMManager {
    async fn create_new_epoch(&mut self, source: String, data: String) -> StorageResult<EpochId> {
        todo!()
    }

    async fn update_stats_from_catalog(
        &self,
        c: storage_layer::CatalogSource,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn update_stats(
        &self,
        stats: i32,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn store_cost(
        &self,
        expr_id: storage_layer::ExprId,
        cost: i32,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn get_stats_analysis(
        &self,
        table_id: i32,
        attr_id: Option<i32>,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_stats(&self, table_id: i32, attr_id: Option<i32>) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_cost_analysis(
        &self,
        expr_id: storage_layer::ExprId,
        epoch_id: storage_layer::EpochId,
    ) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_cost(&self, expr_id: storage_layer::ExprId) -> StorageResult<Option<i32>> {
        todo!()
    }

    async fn get_group_winner_from_group_id(
        &self,
        group_id: i32,
    ) -> StorageResult<Option<physical_expression::ActiveModel>> {
        todo!()
    }

    async fn add_new_expr(
        &mut self,
        expr: storage_layer::Expression,
    ) -> StorageResult<(storage_layer::GroupId, storage_layer::ExprId)> {
        todo!()
    }

    async fn add_expr_to_group(
        &mut self,
        expr: storage_layer::Expression,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Option<storage_layer::ExprId>> {
        todo!()
    }

    async fn get_group_id(
        &self,
        expr_id: storage_layer::ExprId,
    ) -> StorageResult<storage_layer::GroupId> {
        todo!()
    }

    async fn get_expr_memoed(
        &self,
        expr_id: storage_layer::ExprId,
    ) -> StorageResult<storage_layer::Expression> {
        todo!()
    }

    async fn get_all_group_ids(&self) -> StorageResult<Vec<storage_layer::GroupId>> {
        todo!()
    }

    async fn get_group(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<crate::entities::cascades_group::ActiveModel> {
        todo!()
    }

    async fn update_group_winner(
        &mut self,
        group_id: storage_layer::GroupId,
        latest_winner: Option<storage_layer::ExprId>,
    ) -> StorageResult<()> {
        todo!()
    }

    async fn get_all_exprs_in_group(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Vec<storage_layer::ExprId>> {
        todo!()
    }

    async fn get_group_info(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<&Option<storage_layer::ExprId>> {
        todo!()
    }

    async fn get_predicate_binding(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Option<storage_layer::Expression>> {
        todo!()
    }

    async fn try_get_predicate_binding(
        &self,
        group_id: storage_layer::GroupId,
    ) -> StorageResult<Option<storage_layer::Expression>> {
        todo!()
    }
}
