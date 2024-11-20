#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{CatalogSource, Stat, StatType},
    CostModelStorageLayer,
};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PhysicalNodeType, ReprPredicateNode},
        predicates::list_pred::ListPred,
        types::{AttrId, EpochId, ExprId, TableId},
    },
    memo_ext::MemoExt,
    stats::AttributeCombValueStats,
    storage::{self, CostModelStorageManager},
    ComputeCostContext, Cost, CostModel, CostModelResult, EstimatedStatistic, StatValue,
};

/// TODO: documentation
pub struct CostModelImpl<S: CostModelStorageManager> {
    pub storage_manager: S,
    pub default_catalog_source: CatalogSource,
    pub memo: Arc<dyn MemoExt>,
}

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// TODO: documentation
    pub fn new(
        storage_manager: S,
        default_catalog_source: CatalogSource,
        memo: Arc<dyn MemoExt>,
    ) -> Self {
        Self {
            storage_manager,
            default_catalog_source,
            memo,
        }
    }
}

#[async_trait::async_trait]
impl<S: CostModelStorageManager + Send + Sync + 'static> CostModel for CostModelImpl<S> {
    async fn compute_operation_cost(
        &self,
        node: &PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[EstimatedStatistic],
        context: ComputeCostContext,
    ) -> CostModelResult<Cost> {
        todo!()
    }

    async fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[EstimatedStatistic],
        context: ComputeCostContext,
    ) -> CostModelResult<EstimatedStatistic> {
        match node {
            PhysicalNodeType::PhysicalScan => {
                let table_id = TableId(predicates[0].data.as_ref().unwrap().as_u64());
                let row_cnt = self
                    .storage_manager
                    .get_table_row_count(table_id)
                    .await?
                    .unwrap_or(1) as f64;
                Ok(EstimatedStatistic(row_cnt))
            }
            PhysicalNodeType::PhysicalEmptyRelation => Ok(EstimatedStatistic(0.01)),
            PhysicalNodeType::PhysicalLimit => {
                self.get_limit_row_cnt(children_statistics[0].clone(), predicates[1].clone())
            }
            PhysicalNodeType::PhysicalFilter => {
                self.get_filter_row_cnt(
                    children_statistics[0].clone(),
                    context.group_id,
                    predicates[0].clone(),
                )
                .await
            }
            PhysicalNodeType::PhysicalNestedLoopJoin(join_typ) => {
                self.get_nlj_row_cnt(
                    join_typ,
                    context.group_id,
                    children_statistics[0].clone(),
                    children_statistics[1].clone(),
                    context.children_group_ids[0],
                    context.children_group_ids[1],
                    predicates[0].clone(),
                )
                .await
            }
            PhysicalNodeType::PhysicalHashJoin(join_typ) => {
                self.get_hash_join_row_cnt(
                    join_typ,
                    context.group_id,
                    children_statistics[0].clone(),
                    children_statistics[1].clone(),
                    context.children_group_ids[0],
                    context.children_group_ids[1],
                    ListPred::from_pred_node(predicates[0].clone()).unwrap(),
                    ListPred::from_pred_node(predicates[1].clone()).unwrap(),
                )
                .await
            }
            PhysicalNodeType::PhysicalAgg => {
                self.get_agg_row_cnt(context.group_id, predicates[1].clone())
                    .await
            }
            PhysicalNodeType::PhysicalSort | PhysicalNodeType::PhysicalProjection => {
                Ok(children_statistics[0].clone())
            }
        }
    }

    async fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()> {
        todo!()
    }

    async fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    async fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    async fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<crate::Cost>> {
        todo!()
    }
}

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    pub(crate) async fn get_attribute_comb_stats(
        &self,
        table_id: TableId,
        attr_comb: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        self.storage_manager
            .get_attributes_comb_statistics(table_id, attr_comb)
            .await
    }
}
