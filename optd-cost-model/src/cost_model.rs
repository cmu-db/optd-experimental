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
                attr_index_pred::AttrIndexPred,
                bin_op_pred::{BinOpPred, BinOpType},
                cast_pred::CastPred,
                constant_pred::{ConstantPred, ConstantType},
                in_list_pred::InListPred,
                like_pred::LikePred,
                list_pred::ListPred,
                log_op_pred::{LogOpPred, LogOpType},
                un_op_pred::{UnOpPred, UnOpType},
            },
            properties::{
                attr_ref::{AttrRef, GroupAttrRefs},
                schema::Schema,
                Attribute,
            },
            types::GroupId,
            values::Value,
        },
        memo_ext::tests::{MemoGroupInfo, MockMemoExtImpl},
        stats::{
            utilities::counter::Counter, AttributeCombValueStats, Distribution, MostCommonValues,
        },
        storage::mock::{CostModelStorageMockManagerImpl, TableStats},
    };

    use super::*;

    pub const TEST_TABLE1_ID: TableId = TableId(0);
    pub const TEST_TABLE2_ID: TableId = TableId(1);
    pub const TEST_TABLE3_ID: TableId = TableId(2);
    pub const TEST_TABLE4_ID: TableId = TableId(3);

    pub const TEST_GROUP1_ID: GroupId = GroupId(0);
    pub const TEST_GROUP2_ID: GroupId = GroupId(1);
    pub const TEST_GROUP3_ID: GroupId = GroupId(2);
    pub const TEST_GROUP4_ID: GroupId = GroupId(3);

    // This is base index rather than ref index.
    pub const TEST_ATTR1_BASE_INDEX: u64 = 0;
    pub const TEST_ATTR2_BASE_INDEX: u64 = 1;
    pub const TEST_ATTR3_BASE_INDEX: u64 = 2;

    pub type TestPerAttributeStats = AttributeCombValueStats;
    // TODO: add tests for non-mock storage manager
    pub type TestOptCostModelMock = CostModelImpl<CostModelStorageMockManagerImpl>;

    // Use this method, we only create one group `TEST_GROUP1_ID` in the memo.
    // We put the first attribute in the first table as the ref index 0 in the group.
    // And put the second attribute in the first table as the ref index 1 in the group.
    // etc.
    // The orders of attributes and tables are defined by the order of their ids (smaller first).
    pub fn create_mock_cost_model(
        table_id: Vec<TableId>,
        // u64 should be base attribute index.
        per_attribute_stats: Vec<HashMap<u64, TestPerAttributeStats>>,
        row_counts: Vec<Option<u64>>,
    ) -> TestOptCostModelMock {
        let attr_ids: Vec<(TableId, u64, Option<ConstantType>)> = per_attribute_stats
            .iter()
            .enumerate()
            .map(|(idx, m)| (table_id[idx], m))
            .flat_map(|(table_id, m)| {
                m.iter()
                    .map(|(attr_idx, _)| (table_id, *attr_idx, None))
                    .collect_vec()
            })
            .sorted_by_key(|(table_id, attr_idx, _)| (*table_id, *attr_idx))
            .collect();
        create_mock_cost_model_with_memo(
            table_id.clone(),
            per_attribute_stats,
            row_counts,
            create_one_group_all_base_attributes_mock_memo(attr_ids),
        )
    }

    pub fn create_mock_cost_model_with_attr_types(
        table_id: Vec<TableId>,
        // u64 should be base attribute index.
        per_attribute_stats: Vec<HashMap<u64, TestPerAttributeStats>>,
        attributes: Vec<HashMap<u64, ConstantType>>,
        row_counts: Vec<Option<u64>>,
    ) -> TestOptCostModelMock {
        let attr_ids: Vec<(TableId, u64, Option<ConstantType>)> = attributes
            .iter()
            .enumerate()
            .map(|(idx, m)| (table_id[idx], m))
            .flat_map(|(table_id, m)| {
                m.iter()
                    .map(|(attr_idx, typ)| (table_id, *attr_idx, Some(*typ)))
                    .collect_vec()
            })
            .sorted_by_key(|(table_id, attr_idx, _)| (*table_id, *attr_idx))
            .collect();
        create_mock_cost_model_with_memo(
            table_id.clone(),
            per_attribute_stats,
            row_counts,
            create_one_group_all_base_attributes_mock_memo(attr_ids),
        )
    }

    pub fn create_mock_cost_model_with_memo(
        table_id: Vec<TableId>,
        per_attribute_stats: Vec<HashMap<u64, TestPerAttributeStats>>,
        row_counts: Vec<Option<u64>>,
        memo: MockMemoExtImpl,
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
        );
        CostModelImpl::new(storage_manager, CatalogSource::Mock, Arc::new(memo))
    }

    // attributes: Vec<(TableId, AttrBaseIndex)>
    pub fn create_one_group_all_base_attributes_mock_memo(
        attr_ids: Vec<(TableId, u64, Option<ConstantType>)>,
    ) -> MockMemoExtImpl {
        let group_info = MemoGroupInfo::new(
            Schema::new(
                attr_ids
                    .clone()
                    .into_iter()
                    .map(|(_, _, typ)| Attribute {
                        name: "attr".to_string(),
                        typ: typ.unwrap_or(ConstantType::Int64),
                        nullable: false,
                    })
                    .collect(),
            ),
            GroupAttrRefs::new(
                attr_ids
                    .into_iter()
                    .map(|(table_id, attr_base_index, _)| {
                        AttrRef::new_base_table_attr_ref(table_id, attr_base_index)
                    })
                    .collect(),
                None,
            ),
        );
        MockMemoExtImpl::from(HashMap::from([(TEST_GROUP1_ID, group_info)]))
    }

    /// Create a cost model two tables, each with one attribute. Each attribute has 100 values.
    pub fn create_two_table_mock_cost_model(
        tbl1_per_attr_stats: TestPerAttributeStats,
        tbl2_per_attr_stats: TestPerAttributeStats,
    ) -> TestOptCostModelMock {
        create_two_table_cost_model_custom_row_cnts(
            tbl1_per_attr_stats,
            tbl2_per_attr_stats,
            100,
            100,
        )
    }

    /// Create a cost model with three columns, one for each table. Each column has 100 values.
    pub fn create_three_table_cost_model(
        tbl1_per_column_stats: TestPerAttributeStats,
        tbl2_per_column_stats: TestPerAttributeStats,
        tbl3_per_column_stats: TestPerAttributeStats,
    ) -> TestOptCostModelMock {
        let storage_manager = CostModelStorageMockManagerImpl::new(
            vec![
                (
                    TEST_TABLE1_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl1_per_column_stats)].into_iter().collect(),
                    ),
                ),
                (
                    TEST_TABLE2_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl2_per_column_stats)].into_iter().collect(),
                    ),
                ),
                (
                    TEST_TABLE3_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl3_per_column_stats)].into_iter().collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let memo = HashMap::from([
            (
                TEST_GROUP1_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr1".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP2_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr2".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP3_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr3".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE3_ID, 0)],
                        None,
                    ),
                ),
            ),
        ]);
        CostModelImpl::new(
            storage_manager,
            CatalogSource::Mock,
            Arc::new(MockMemoExtImpl::from(memo)),
        )
    }

    /// Create a cost model with three columns, one for each table. Each column has 100 values.
    pub fn create_four_table_cost_model(
        tbl1_per_column_stats: TestPerAttributeStats,
        tbl2_per_column_stats: TestPerAttributeStats,
        tbl3_per_column_stats: TestPerAttributeStats,
        tbl4_per_column_stats: TestPerAttributeStats,
    ) -> TestOptCostModelMock {
        let storage_manager = CostModelStorageMockManagerImpl::new(
            vec![
                (
                    TEST_TABLE1_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl1_per_column_stats)].into_iter().collect(),
                    ),
                ),
                (
                    TEST_TABLE2_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl2_per_column_stats)].into_iter().collect(),
                    ),
                ),
                (
                    TEST_TABLE3_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl3_per_column_stats)].into_iter().collect(),
                    ),
                ),
                (
                    TEST_TABLE4_ID,
                    TableStats::new(
                        100,
                        vec![(vec![0], tbl4_per_column_stats)].into_iter().collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let memo = HashMap::from([
            (
                TEST_GROUP1_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr1".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP2_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr2".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP3_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr3".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE3_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP4_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr4".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE4_ID, 0)],
                        None,
                    ),
                ),
            ),
        ]);
        CostModelImpl::new(
            storage_manager,
            CatalogSource::Mock,
            Arc::new(MockMemoExtImpl::from(memo)),
        )
    }

    /// We need custom row counts because some join algorithms rely on the row cnt
    pub fn create_two_table_cost_model_custom_row_cnts(
        tbl1_per_column_stats: TestPerAttributeStats,
        tbl2_per_column_stats: TestPerAttributeStats,
        tbl1_row_cnt: u64,
        tbl2_row_cnt: u64,
    ) -> TestOptCostModelMock {
        let storage_manager = CostModelStorageMockManagerImpl::new(
            vec![
                (
                    TEST_TABLE1_ID,
                    TableStats::new(
                        tbl1_row_cnt,
                        vec![(vec![0], tbl1_per_column_stats)].into_iter().collect(),
                    ),
                ),
                (
                    TEST_TABLE2_ID,
                    TableStats::new(
                        tbl2_row_cnt,
                        vec![(vec![0], tbl2_per_column_stats)].into_iter().collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let memo = HashMap::from([
            (
                TEST_GROUP1_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr1".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP2_ID,
                MemoGroupInfo::new(
                    vec![Attribute {
                        name: "attr2".to_string(),
                        typ: ConstantType::Int64,
                        nullable: false,
                    }]
                    .into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0)],
                        None,
                    ),
                ),
            ),
        ]);
        CostModelImpl::new(
            storage_manager,
            CatalogSource::Mock,
            Arc::new(MockMemoExtImpl::from(memo)),
        )
    }

    impl TestOptCostModelMock {
        pub fn get_row_count(&self, table_id: TableId) -> u64 {
            self.storage_manager
                .per_table_stats_map
                .get(&table_id)
                .map(|stats| stats.row_cnt)
                .unwrap_or(0)
        }

        pub fn get_attr_refs(&self, group_id: GroupId) -> GroupAttrRefs {
            self.memo.get_attribute_refs(group_id)
        }
    }

    pub fn attr_index(attr_index: u64) -> ArcPredicateNode {
        AttrIndexPred::new(attr_index).into_pred_node()
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

    pub fn in_list(attr_idx: u64, list: Vec<Value>, negated: bool) -> InListPred {
        InListPred::new(
            attr_index(attr_idx),
            ListPred::new(list.into_iter().map(cnst).collect_vec()),
            negated,
        )
    }

    pub fn like(attr_idx: u64, pattern: &str, negated: bool) -> LikePred {
        LikePred::new(
            negated,
            false,
            attr_index(attr_idx),
            cnst(Value::String(pattern.into())),
        )
    }

    pub(crate) fn empty_per_attr_stats() -> TestPerAttributeStats {
        TestPerAttributeStats::new(MostCommonValues::Counter(Counter::default()), None, 0, 0.0)
    }
}
