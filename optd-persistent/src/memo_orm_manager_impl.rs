#![allow(dead_code, unused_imports, unused_variables)]

use crate::memo_storage_layer::MemoStorageLayer;
use crate::orm_manager::ORMManager;

impl MemoStorageLayer for ORMManager {
    async fn get_group_winner_from_group_id(
        &self,
        group_id: i32,
    ) -> crate::StorageResult<Option<crate::entities::physical_expression::ActiveModel>> {
        todo!()
    }

    async fn add_new_expr(
        &mut self,
        expr: crate::memo_storage_layer::Expression,
    ) -> crate::StorageResult<(crate::GroupId, crate::ExprId)> {
        todo!()
    }

    async fn add_expr_to_group(
        &mut self,
        expr: crate::memo_storage_layer::Expression,
        group_id: crate::GroupId,
    ) -> crate::StorageResult<Option<crate::ExprId>> {
        todo!()
    }

    async fn get_group_id(&self, expr_id: crate::ExprId) -> crate::StorageResult<crate::GroupId> {
        todo!()
    }

    async fn get_expr_memoed(
        &self,
        expr_id: crate::ExprId,
    ) -> crate::StorageResult<crate::memo_storage_layer::Expression> {
        todo!()
    }

    async fn get_all_group_ids(&self) -> crate::StorageResult<Vec<crate::GroupId>> {
        todo!()
    }

    async fn get_group(
        &self,
        group_id: crate::GroupId,
    ) -> crate::StorageResult<crate::entities::cascades_group::ActiveModel> {
        todo!()
    }

    async fn update_group_winner(
        &mut self,
        group_id: crate::GroupId,
        latest_winner: Option<crate::ExprId>,
    ) -> crate::StorageResult<()> {
        todo!()
    }

    async fn get_all_exprs_in_group(
        &self,
        group_id: crate::GroupId,
    ) -> crate::StorageResult<Vec<crate::ExprId>> {
        todo!()
    }

    async fn get_group_info(
        &self,
        group_id: crate::GroupId,
    ) -> crate::StorageResult<&Option<crate::ExprId>> {
        todo!()
    }

    async fn get_predicate_binding(
        &self,
        group_id: crate::GroupId,
    ) -> crate::StorageResult<Option<crate::memo_storage_layer::Expression>> {
        todo!()
    }

    async fn try_get_predicate_binding(
        &self,
        group_id: crate::GroupId,
    ) -> crate::StorageResult<Option<crate::memo_storage_layer::Expression>> {
        todo!()
    }
}
