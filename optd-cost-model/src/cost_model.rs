#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{CatalogSource, Stat, StatType},
    CostModelStorageLayer,
};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PhysicalNodeType},
        types::{AttrId, EpochId, ExprId, TableId},
    },
    memo_ext::MemoExt,
    stats::AttributeCombValueStats,
    storage::CostModelStorageManager,
    ComputeCostContext, Cost, CostModel, CostModelResult, EstimatedStatistic, StatValue,
};

/// TODO: documentation
pub struct CostModelImpl<S: CostModelStorageManager> {
    pub storage_manager: S,
    pub default_catalog_source: CatalogSource,
    _memo: Arc<dyn MemoExt>,
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
            _memo: memo,
        }
    }
}

impl<S: CostModelStorageManager + Send + Sync + 'static> CostModel for CostModelImpl<S> {
    fn compute_operation_cost(
        &self,
        node: &PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[Option<&EstimatedStatistic>],
        context: ComputeCostContext,
    ) -> CostModelResult<Cost> {
        todo!()
    }

    fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[Option<&EstimatedStatistic>],
        context: ComputeCostContext,
    ) -> CostModelResult<EstimatedStatistic> {
        todo!()
    }

    fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()> {
        todo!()
    }

    fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    fn get_cost_for_analysis(
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

/// I thought about using the system's own parser and planner to generate these expression trees,
/// but this is not currently feasible because it would create a cyclic dependency between
/// optd-datafusion-bridge and optd-datafusion-repr
#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use arrow_schema::DataType;
    use itertools::Itertools;
    use optd_persistent::cost_model::interface::CatalogSource;
    use serde::{Deserialize, Serialize};

    use crate::{
        common::{
            nodes::ReprPredicateNode,
            predicates::{
                attr_ref_pred::AttrRefPred,
                bin_op_pred::{BinOpPred, BinOpType},
                cast_pred::CastPred,
                constant_pred::ConstantPred,
                in_list_pred::InListPred,
                like_pred::LikePred,
                list_pred::ListPred,
                log_op_pred::{LogOpPred, LogOpType},
                un_op_pred::{UnOpPred, UnOpType},
            },
            values::Value,
        },
        memo_ext::tests::MockMemoExt,
        stats::{
            utilities::counter::Counter, AttributeCombValueStats, Distribution, MostCommonValues,
        },
        storage::mock::{BaseTableAttrInfo, CostModelStorageMockManagerImpl, TableStats},
    };

    use super::*;

    pub type TestPerAttributeStats = AttributeCombValueStats;
    // TODO: add tests for non-mock storage manager
    pub type TestOptCostModelMock = CostModelImpl<CostModelStorageMockManagerImpl>;

    pub fn create_cost_model_mock_storage(
        table_id: Vec<TableId>,
        per_attribute_stats: Vec<HashMap<u64, TestPerAttributeStats>>,
        row_counts: Vec<Option<u64>>,
        per_table_attr_infos: BaseTableAttrInfo,
    ) -> TestOptCostModelMock {
        let storage_manager = CostModelStorageMockManagerImpl::new(
            table_id
                .into_iter()
                .zip(per_attribute_stats)
                .zip(row_counts)
                .map(|((table_id, per_attr_stats), row_count)| {
                    (
                        table_id,
                        TableStats::new(
                            row_count.unwrap_or(100),
                            per_attr_stats
                                .into_iter()
                                .map(|(attr_idx, stats)| (vec![attr_idx], stats))
                                .collect(),
                        ),
                    )
                })
                .collect(),
            per_table_attr_infos,
        );
        CostModelImpl::new(storage_manager, CatalogSource::Mock, Arc::new(MockMemoExt))
    }

    pub fn attr_ref(table_id: TableId, attr_base_index: u64) -> ArcPredicateNode {
        AttrRefPred::new(table_id, attr_base_index).into_pred_node()
    }

    pub fn cnst(value: Value) -> ArcPredicateNode {
        ConstantPred::new(value).into_pred_node()
    }

    pub fn cast(child: ArcPredicateNode, cast_type: DataType) -> ArcPredicateNode {
        CastPred::new(child, cast_type).into_pred_node()
    }

    pub fn bin_op(
        op_type: BinOpType,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
    ) -> ArcPredicateNode {
        BinOpPred::new(left, right, op_type).into_pred_node()
    }

    pub fn log_op(op_type: LogOpType, children: Vec<ArcPredicateNode>) -> ArcPredicateNode {
        LogOpPred::new(op_type, children).into_pred_node()
    }

    pub fn un_op(op_type: UnOpType, child: ArcPredicateNode) -> ArcPredicateNode {
        UnOpPred::new(child, op_type).into_pred_node()
    }

    pub fn empty_list() -> ArcPredicateNode {
        ListPred::new(vec![]).into_pred_node()
    }

    pub fn list(children: Vec<ArcPredicateNode>) -> ArcPredicateNode {
        ListPred::new(children).into_pred_node()
    }

    pub fn in_list(
        table_id: TableId,
        attr_ref_idx: u64,
        list: Vec<Value>,
        negated: bool,
    ) -> InListPred {
        InListPred::new(
            attr_ref(table_id, attr_ref_idx),
            ListPred::new(list.into_iter().map(cnst).collect_vec()),
            negated,
        )
    }

    pub fn like(table_id: TableId, attr_ref_idx: u64, pattern: &str, negated: bool) -> LikePred {
        LikePred::new(
            negated,
            false,
            attr_ref(table_id, attr_ref_idx),
            cnst(Value::String(pattern.into())),
        )
    }

    pub(crate) fn empty_per_attr_stats() -> TestPerAttributeStats {
        TestPerAttributeStats::new(MostCommonValues::Counter(Counter::default()), None, 0, 0.0)
    }
}
