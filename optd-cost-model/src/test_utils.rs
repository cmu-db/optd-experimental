/// I thought about using the system's own parser and planner to generate these expression trees,
/// but this is not currently feasible because it would create a cyclic dependency between
/// optd-datafusion-bridge and optd-datafusion-repr

#[cfg(any(test, feature = "include-tests"))]
pub mod tests {
    use itertools::Itertools;
    use std::{collections::HashMap, sync::Arc};

    use arrow_schema::DataType;
    use optd_persistent::cost_model::interface::CatalogSource;

    use crate::{
        common::{
            nodes::{ArcPredicateNode, ReprPredicateNode},
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
            types::{GroupId, TableId},
            values::Value,
        },
        cost_model::CostModelImpl,
        memo_ext::MemoExt,
        stats::{
            utilities::simple_map::SimpleMap, AttributeCombValueStats, Distribution,
            MostCommonValues,
        },
        storage::mock::{CostModelStorageMockManagerImpl, TableStats},
    };

    pub struct MemoGroupInfo {
        pub schema: Schema,
        pub attr_refs: GroupAttrRefs,
    }

    impl MemoGroupInfo {
        pub fn new(schema: Schema, attr_refs: GroupAttrRefs) -> Self {
            Self { schema, attr_refs }
        }
    }

    #[derive(Default)]
    pub struct MockMemoExtImpl {
        memo: HashMap<GroupId, MemoGroupInfo>,
    }

    impl MockMemoExtImpl {
        pub fn add_group_info(
            &mut self,
            group_id: GroupId,
            schema: Schema,
            attr_ref: GroupAttrRefs,
        ) {
            self.memo
                .insert(group_id, MemoGroupInfo::new(schema, attr_ref));
        }
    }

    impl MemoExt for MockMemoExtImpl {
        fn get_schema(&self, group_id: GroupId) -> Schema {
            self.memo.get(&group_id).unwrap().schema.clone()
        }

        fn get_attribute_info(&self, group_id: GroupId, attr_ref_idx: u64) -> Attribute {
            self.memo.get(&group_id).unwrap().schema.attributes[attr_ref_idx as usize].clone()
        }

        fn get_attribute_refs(&self, group_id: GroupId) -> GroupAttrRefs {
            self.memo.get(&group_id).unwrap().attr_refs.clone()
        }

        fn get_attribute_ref(&self, group_id: GroupId, attr_ref_idx: u64) -> AttrRef {
            self.memo.get(&group_id).unwrap().attr_refs.attr_refs()[attr_ref_idx as usize].clone()
        }
    }

    impl From<HashMap<GroupId, MemoGroupInfo>> for MockMemoExtImpl {
        fn from(memo: HashMap<GroupId, MemoGroupInfo>) -> Self {
            Self { memo }
        }
    }

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

    pub const TEST_ATTR1_NAME: &str = "attr1";
    pub const TEST_ATTR2_NAME: &str = "attr2";
    pub const TEST_ATTR3_NAME: &str = "attr3";
    pub const TEST_ATTR4_NAME: &str = "attr4";

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
        additional_memo: Option<HashMap<GroupId, MemoGroupInfo>>,
    ) -> TestOptCostModelMock {
        create_two_table_mock_cost_model_custom_row_cnts(
            tbl1_per_attr_stats,
            tbl2_per_attr_stats,
            100,
            100,
            additional_memo,
        )
    }

    /// Create a cost model three tables, each with one attribute. Each attribute has 100 values.
    pub fn create_three_table_mock_cost_model(
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
                    vec![Attribute::new_non_null_int64(TEST_ATTR1_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP2_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR2_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP3_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR3_NAME.to_string())].into(),
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

    /// Create a cost model four tables, each with one attribute. Each attribute has 100 values.
    pub fn create_four_table_mock_cost_model(
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
                    vec![Attribute::new_non_null_int64(TEST_ATTR1_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP2_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR2_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP3_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR3_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE3_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP4_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR4_NAME.to_string())].into(),
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
    pub fn create_two_table_mock_cost_model_custom_row_cnts(
        tbl1_per_column_stats: TestPerAttributeStats,
        tbl2_per_column_stats: TestPerAttributeStats,
        tbl1_row_cnt: u64,
        tbl2_row_cnt: u64,
        additional_memo: Option<HashMap<GroupId, MemoGroupInfo>>,
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
        let mut memo = HashMap::from([
            (
                TEST_GROUP1_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR1_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0)],
                        None,
                    ),
                ),
            ),
            (
                TEST_GROUP2_ID,
                MemoGroupInfo::new(
                    vec![Attribute::new_non_null_int64(TEST_ATTR2_NAME.to_string())].into(),
                    GroupAttrRefs::new(
                        vec![AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0)],
                        None,
                    ),
                ),
            ),
        ]);
        if let Some(additional_memo) = additional_memo {
            memo.extend(additional_memo);
        }
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

    pub fn empty_per_attr_stats() -> TestPerAttributeStats {
        TestPerAttributeStats::new(
            MostCommonValues::empty(),
            Some(Distribution::empty()),
            0,
            0.0,
        )
    }

    pub fn per_attr_stats_with_ndistinct(ndistinct: u64) -> TestPerAttributeStats {
        TestPerAttributeStats::new(
            MostCommonValues::empty(),
            Some(Distribution::empty()),
            ndistinct,
            0.0,
        )
    }

    pub fn per_attr_stats_with_dist_and_ndistinct(
        dist: Vec<(Value, f64)>,
        ndistinct: u64,
    ) -> TestPerAttributeStats {
        TestPerAttributeStats::new(
            MostCommonValues::empty(),
            Some(Distribution::SimpleDistribution(SimpleMap::new(dist))),
            ndistinct,
            0.0,
        )
    }
}
