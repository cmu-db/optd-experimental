use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    common::{
        nodes::{ArcPredicateNode, JoinType, PredicateType, ReprPredicateNode},
        predicates::{
            attr_index_pred::AttrIndexPred,
            list_pred::ListPred,
            log_op_pred::{LogOpPred, LogOpType},
        },
        properties::attr_ref::{
            AttrRef, AttrRefs, BaseTableAttrRef, EqPredicate, SemanticCorrelation,
        },
        types::GroupId,
    },
    cost::join::get_on_attr_ref_pair,
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// The expr_tree input must be a "mixed expression tree", just like with
    /// `get_filter_selectivity`.
    ///
    /// This is a "wrapper" to separate the equality conditions from the filter conditions before
    /// calling the "main" `get_join_selectivity_core` function.
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn get_join_selectivity_from_expr_tree(
        &self,
        join_typ: JoinType,
        group_id: GroupId,
        expr_tree: ArcPredicateNode,
        attr_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        left_row_cnt: f64,
        right_row_cnt: f64,
    ) -> CostModelResult<f64> {
        if expr_tree.typ == PredicateType::LogOp(LogOpType::And) {
            let mut on_attr_ref_pairs = vec![];
            let mut filter_expr_trees = vec![];
            for child_expr_tree in &expr_tree.children {
                if let Some(on_attr_ref_pair) =
                    get_on_attr_ref_pair(child_expr_tree.clone(), attr_refs)
                {
                    on_attr_ref_pairs.push(on_attr_ref_pair)
                } else {
                    let child_expr = child_expr_tree.clone();
                    filter_expr_trees.push(child_expr);
                }
            }
            assert!(on_attr_ref_pairs.len() + filter_expr_trees.len() == expr_tree.children.len());
            let filter_expr_tree = if filter_expr_trees.is_empty() {
                None
            } else {
                Some(LogOpPred::new(LogOpType::And, filter_expr_trees).into_pred_node())
            };
            self.get_join_selectivity_core(
                join_typ,
                group_id,
                on_attr_ref_pairs,
                filter_expr_tree,
                attr_refs,
                input_correlation,
                left_row_cnt,
                right_row_cnt,
                0,
            )
            .await
        } else {
            #[allow(clippy::collapsible_else_if)]
            if let Some(on_attr_ref_pair) = get_on_attr_ref_pair(expr_tree.clone(), attr_refs) {
                self.get_join_selectivity_core(
                    join_typ,
                    group_id,
                    vec![on_attr_ref_pair],
                    None,
                    attr_refs,
                    input_correlation,
                    left_row_cnt,
                    right_row_cnt,
                    0,
                )
                .await
            } else {
                self.get_join_selectivity_core(
                    join_typ,
                    group_id,
                    vec![],
                    Some(expr_tree),
                    attr_refs,
                    input_correlation,
                    left_row_cnt,
                    right_row_cnt,
                    0,
                )
                .await
            }
        }
    }

    /// A wrapper to convert the join keys to the format expected by get_join_selectivity_core()
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn get_join_selectivity_from_keys(
        &self,
        join_typ: JoinType,
        group_id: GroupId,
        left_keys: ListPred,
        right_keys: ListPred,
        attr_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        left_row_cnt: f64,
        right_row_cnt: f64,
        left_attr_cnt: usize,
    ) -> CostModelResult<f64> {
        assert!(left_keys.len() == right_keys.len());
        // I assume that the keys are already in the right order
        // s.t. the ith key of left_keys corresponds with the ith key of right_keys
        let on_attr_ref_pairs = left_keys
            .to_vec()
            .into_iter()
            .zip(right_keys.to_vec())
            .map(|(left_key, right_key)| {
                (
                    AttrIndexPred::from_pred_node(left_key).expect("keys should be AttrRefPreds"),
                    AttrIndexPred::from_pred_node(right_key).expect("keys should be AttrRefPreds"),
                )
            })
            .collect_vec();
        self.get_join_selectivity_core(
            join_typ,
            group_id,
            on_attr_ref_pairs,
            None,
            attr_refs,
            input_correlation,
            left_row_cnt,
            right_row_cnt,
            left_attr_cnt,
        )
        .await
    }

    /// The core logic of join selectivity which assumes we've already separated the expression
    /// into the on conditions and the filters.
    ///
    /// Hash join and NLJ reference right table attributes differently, hence the
    /// `right_attr_ref_offset` parameter.
    ///
    /// For hash join, the right table attributes indices are with respect to the right table,
    /// which means #0 is the first attribute of the right table.
    ///
    /// For NLJ, the right table attributes indices are with respect to the output of the join.
    /// For example, if the left table has 3 attributes, the first attribute of the right table
    /// is #3 instead of #0.
    #[allow(clippy::too_many_arguments)]
    async fn get_join_selectivity_core(
        &self,
        join_typ: JoinType,
        group_id: GroupId,
        on_attr_ref_pairs: Vec<(AttrIndexPred, AttrIndexPred)>,
        filter_expr_tree: Option<ArcPredicateNode>,
        attr_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        left_row_cnt: f64,
        right_row_cnt: f64,
        right_attr_ref_offset: usize,
    ) -> CostModelResult<f64> {
        let join_on_selectivity = self
            .get_join_on_selectivity(
                &on_attr_ref_pairs,
                attr_refs,
                input_correlation,
                right_attr_ref_offset,
            )
            .await?;
        // Currently, there is no difference in how we handle a join filter and a select filter,
        // so we use the same function.
        //
        // One difference (that we *don't* care about right now) is that join filters can contain
        // expressions from multiple different tables. Currently, this doesn't affect the
        // get_filter_selectivity() function, but this may change in the future.
        let join_filter_selectivity = match filter_expr_tree {
            Some(filter_expr_tree) => {
                self.get_filter_selectivity(group_id, filter_expr_tree)
                    .await?
            }
            None => 1.0,
        };
        let inner_join_selectivity = join_on_selectivity * join_filter_selectivity;

        Ok(match join_typ {
            JoinType::Inner => inner_join_selectivity,
            JoinType::LeftOuter => f64::max(inner_join_selectivity, 1.0 / right_row_cnt),
            JoinType::RightOuter => f64::max(inner_join_selectivity, 1.0 / left_row_cnt),
            JoinType::Cross => {
                assert!(
                    on_attr_ref_pairs.is_empty(),
                    "Cross joins should not have on attributes"
                );
                join_filter_selectivity
            }
            _ => unimplemented!("join_typ={} is not implemented", join_typ),
        })
    }

    /// Get the selectivity of one attribute eq predicate, e.g. attrA = attrB.
    async fn get_join_selectivity_from_on_attr_ref_pair(
        &self,
        left: &AttrRef,
        right: &AttrRef,
    ) -> CostModelResult<f64> {
        // the formula for each pair is min(1 / ndistinct1, 1 / ndistinct2)
        // (see https://postgrespro.com/blog/pgsql/5969618)
        let mut ndistincts = vec![];
        for attr_ref in [left, right] {
            let ndistinct = match attr_ref {
                AttrRef::BaseTableAttrRef(base_attr_ref) => {
                    match self
                        .get_attribute_comb_stats(base_attr_ref.table_id, &[base_attr_ref.attr_idx])
                        .await?
                    {
                        Some(per_attr_stats) => per_attr_stats.ndistinct,
                        None => DEFAULT_NUM_DISTINCT,
                    }
                }
                AttrRef::Derived => DEFAULT_NUM_DISTINCT,
            };
            ndistincts.push(ndistinct);
        }

        // using reduce(f64::min) is the idiomatic workaround to min() because
        // f64 does not implement Ord due to NaN
        let selectivity = ndistincts.into_iter().map(|ndistinct| 1.0 / ndistinct as f64).reduce(f64::min).expect("reduce() only returns None if the iterator is empty, which is impossible since attr_ref_exprs.len() == 2");
        assert!(
            !selectivity.is_nan(),
            "it should be impossible for selectivity to be NaN since n-distinct is never 0"
        );
        Ok(selectivity)
    }

    /// Given a set of N attributes involved in a multi-equality, find the total selectivity
    /// of the multi-equality.
    ///
    /// This is a generalization of get_join_selectivity_from_on_attr_ref_pair().
    async fn get_join_selectivity_from_most_selective_attrs(
        &self,
        base_attr_refs: HashSet<BaseTableAttrRef>,
    ) -> CostModelResult<f64> {
        assert!(base_attr_refs.len() > 1);
        let num_base_attr_refs = base_attr_refs.len();

        let mut ndistincts = vec![];
        for base_attr_ref in base_attr_refs.iter() {
            let ndistinct = match self
                .get_attribute_comb_stats(base_attr_ref.table_id, &[base_attr_ref.attr_idx])
                .await?
            {
                Some(per_attr_stats) => per_attr_stats.ndistinct,
                None => DEFAULT_NUM_DISTINCT,
            };
            ndistincts.push(ndistinct);
        }

        Ok(ndistincts
            .into_iter()
            .map(|ndistinct| 1.0 / ndistinct as f64)
            .sorted_by(|a, b| {
                a.partial_cmp(b)
                    .expect("No floats should be NaN since n-distinct is never 0")
            })
            .take(num_base_attr_refs - 1)
            .product())
    }

    /// A predicate set defines a "multi-equality graph", which is an unweighted undirected graph.
    /// The nodes are attributes while edges are predicates. The old graph is defined by
    /// `past_eq_attrs` while the `predicate` is the new addition to this graph. This
    /// unweighted undirected graph consists of a number of connected components, where each
    /// connected component represents attributes that are set to be equal to each other. Single
    /// nodes not connected to anything are considered standalone connected components.
    ///
    /// The selectivity of each connected component of N nodes is equal to the product of
    /// 1/ndistinct of the N-1 nodes with the highest ndistinct values. You can see this if you
    /// imagine that all attributes being joined are unique attributes and that they follow the
    /// inclusion principle (every element of the smaller tables is present in the larger
    /// tables). When these assumptions are not true, the selectivity may not be completely
    /// accurate. However, it is still fairly accurate.
    ///
    /// However, we cannot simply add `predicate` to the multi-equality graph and compute the
    /// selectivity of the entire connected component, because this would be "double counting" a
    /// lot of nodes. The join(s) before this join would already have a selectivity value. Thus,
    /// we compute the selectivity of the join(s) before this join (the first block of the
    /// function) and then the selectivity of the connected component after this join. The
    /// quotient is the "adjustment" factor.
    ///
    /// NOTE: This function modifies `past_eq_attrs` by adding `predicate` to it.
    async fn get_join_selectivity_adjustment_when_adding_to_multi_equality_graph(
        &self,
        predicate: &EqPredicate,
        past_eq_attrs: &mut SemanticCorrelation,
    ) -> CostModelResult<f64> {
        if predicate.left == predicate.right {
            // self-join, TODO: is this correct?
            return Ok(1.0);
        }
        // To find the adjustment, we need to know the selectivity of the graph before `predicate`
        // is added.
        //
        // There are two cases: (1) adding `predicate` does not change the # of connected
        // components, and (2) adding `predicate` reduces the # of connected by 1. Note that
        // attributes not involved in any predicates are considered a part of the graph and are
        // a connected component on their own.
        let children_pred_sel = {
            if past_eq_attrs.is_eq(&predicate.left, &predicate.right) {
                self.get_join_selectivity_from_most_selective_attrs(
                    past_eq_attrs.find_attrs_for_eq_attribute_set(&predicate.left),
                )
                .await?
            } else {
                let left_sel = if past_eq_attrs.contains(&predicate.left) {
                    self.get_join_selectivity_from_most_selective_attrs(
                        past_eq_attrs.find_attrs_for_eq_attribute_set(&predicate.left),
                    )
                    .await?
                } else {
                    1.0
                };
                let right_sel = if past_eq_attrs.contains(&predicate.right) {
                    self.get_join_selectivity_from_most_selective_attrs(
                        past_eq_attrs.find_attrs_for_eq_attribute_set(&predicate.right),
                    )
                    .await?
                } else {
                    1.0
                };
                left_sel * right_sel
            }
        };

        // Add predicate to past_eq_attrs and compute the selectivity of the connected component
        // it creates.
        past_eq_attrs.add_predicate(predicate.clone());
        let new_pred_sel = {
            let attrs = past_eq_attrs.find_attrs_for_eq_attribute_set(&predicate.left);
            self.get_join_selectivity_from_most_selective_attrs(attrs)
        }
        .await?;

        // Compute the adjustment factor.
        Ok(new_pred_sel / children_pred_sel)
    }

    /// Get the selectivity of the on conditions.
    ///
    /// Note that the selectivity of the on conditions does not depend on join type.
    /// Join type is accounted for separately in get_join_selectivity_core().
    ///
    /// We also check if each predicate is correlated with any of the previous predicates.
    ///
    /// More specifically, we are checking if the predicate can be expressed with other existing
    /// predicates. E.g. if we have a predicate like A = B and B = C is equivalent to A = C.
    //
    /// However, we don't just throw away A = C, because we want to pick the most selective
    /// predicates. For details on how we do this, see
    /// `get_join_selectivity_from_redundant_predicates`.
    async fn get_join_on_selectivity(
        &self,
        on_attr_ref_pairs: &[(AttrIndexPred, AttrIndexPred)],
        attr_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        right_attr_ref_offset: usize,
    ) -> CostModelResult<f64> {
        let mut past_eq_attrs = input_correlation.unwrap_or_default();

        // Multiply the selectivities of all individual conditions together
        let mut selectivity = 1.0;
        for on_attr_ref_pair in on_attr_ref_pairs {
            let left_attr_ref = &attr_refs[on_attr_ref_pair.0.attr_index() as usize];
            let right_attr_ref =
                &attr_refs[on_attr_ref_pair.1.attr_index() as usize + right_attr_ref_offset];

            selectivity *=
                if let (AttrRef::BaseTableAttrRef(left), AttrRef::BaseTableAttrRef(right)) =
                    (left_attr_ref, right_attr_ref)
                {
                    let predicate = EqPredicate::new(left.clone(), right.clone());
                    self.get_join_selectivity_adjustment_when_adding_to_multi_equality_graph(
                        &predicate,
                        &mut past_eq_attrs,
                    )
                    .await?
                } else {
                    self.get_join_selectivity_from_on_attr_ref_pair(left_attr_ref, right_attr_ref)
                        .await?
                };
        }

        Ok(selectivity)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use attr_ref::GroupAttrRefs;

    use crate::{
        common::{
            predicates::bin_op_pred::BinOpType,
            properties::{attr_ref, Attribute},
            values::Value,
        },
        cost_model::tests::{
            attr_index, bin_op, cnst, create_four_table_mock_cost_model, create_mock_cost_model,
            create_three_table_mock_cost_model, create_two_table_mock_cost_model,
            create_two_table_mock_cost_model_custom_row_cnts, empty_per_attr_stats, log_op,
            per_attr_stats_with_dist_and_ndistinct, per_attr_stats_with_ndistinct,
            TestOptCostModelMock, TEST_ATTR1_NAME, TEST_ATTR2_NAME, TEST_TABLE1_ID, TEST_TABLE2_ID,
            TEST_TABLE3_ID, TEST_TABLE4_ID,
        },
        memo_ext::tests::MemoGroupInfo,
        stats::DEFAULT_EQ_SEL,
    };

    use super::*;

    const JOIN_GROUP_ID: GroupId = GroupId(10);

    /// A wrapper around get_join_selectivity_from_expr_tree that extracts the
    /// table row counts from the cost model.
    async fn test_get_join_selectivity(
        cost_model: &TestOptCostModelMock,
        reverse_tables: bool,
        join_typ: JoinType,
        expr_tree: ArcPredicateNode,
        attr_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
    ) -> f64 {
        let table1_row_cnt = cost_model.get_row_count(TEST_TABLE1_ID) as f64;
        let table2_row_cnt = cost_model.get_row_count(TEST_TABLE2_ID) as f64;

        if !reverse_tables {
            cost_model
                .get_join_selectivity_from_expr_tree(
                    join_typ,
                    JOIN_GROUP_ID,
                    expr_tree,
                    attr_refs,
                    input_correlation,
                    table1_row_cnt,
                    table2_row_cnt,
                )
                .await
                .unwrap()
        } else {
            cost_model
                .get_join_selectivity_from_expr_tree(
                    join_typ,
                    JOIN_GROUP_ID,
                    expr_tree,
                    attr_refs,
                    input_correlation,
                    table2_row_cnt,
                    table1_row_cnt,
                )
                .await
                .unwrap()
        }
    }

    #[tokio::test]
    async fn test_inner_const() {
        let cost_model = create_mock_cost_model(
            vec![TEST_TABLE1_ID],
            vec![HashMap::from([(0, empty_per_attr_stats())])],
            vec![None],
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_join_selectivity_from_expr_tree(
                    JoinType::Inner,
                    JOIN_GROUP_ID,
                    cnst(Value::Bool(true)),
                    &vec![],
                    None,
                    f64::NAN,
                    f64::NAN
                )
                .await
                .unwrap(),
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_join_selectivity_from_expr_tree(
                    JoinType::Inner,
                    JOIN_GROUP_ID,
                    cnst(Value::Bool(false)),
                    &vec![],
                    None,
                    f64::NAN,
                    f64::NAN
                )
                .await
                .unwrap(),
            0.0
        );
    }

    #[tokio::test]
    async fn test_inner_oncond() {
        let cost_model = create_two_table_mock_cost_model(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            None,
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        let expr_tree = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let expr_tree_rev = bin_op(BinOpType::Eq, attr_index(1), attr_index(0));
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree,
                &attr_refs,
                None,
            )
            .await,
            0.2
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev,
                &attr_refs,
                None,
            )
            .await,
            0.2
        );
    }

    #[tokio::test]
    async fn test_inner_and_of_onconds() {
        let cost_model = create_two_table_mock_cost_model(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            None,
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        let eq0and1 = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let eq1and0 = bin_op(BinOpType::Eq, attr_index(1), attr_index(0));
        let expr_tree = log_op(LogOpType::And, vec![eq0and1.clone(), eq1and0.clone()]);
        let expr_tree_rev = log_op(LogOpType::And, vec![eq1and0.clone(), eq0and1.clone()]);

        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree,
                &attr_refs,
                None,
            )
            .await,
            0.2
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev,
                &attr_refs,
                None
            )
            .await,
            0.2
        );
    }

    #[tokio::test]
    async fn test_inner_and_of_oncond_and_filter() {
        let join_memo = HashMap::from([(
            JOIN_GROUP_ID,
            MemoGroupInfo::new(
                vec![
                    Attribute::new_non_null_int64(TEST_ATTR1_NAME.to_string()),
                    Attribute::new_non_null_int64(TEST_ATTR2_NAME.to_string()),
                ]
                .into(),
                GroupAttrRefs::new(
                    vec![
                        AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0),
                        AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0),
                    ],
                    None,
                ),
            ),
        )]);
        let cost_model = create_two_table_mock_cost_model(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            Some(join_memo),
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        let eq0and1 = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let eq100 = bin_op(BinOpType::Eq, attr_index(1), cnst(Value::Int32(100)));
        let expr_tree = log_op(LogOpType::And, vec![eq0and1.clone(), eq100.clone()]);
        let expr_tree_rev = log_op(LogOpType::And, vec![eq100.clone(), eq0and1.clone()]);

        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree,
                &attr_refs,
                None
            )
            .await,
            0.05
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev,
                &attr_refs,
                None
            )
            .await,
            0.05
        );
    }

    #[tokio::test]
    async fn test_inner_and_of_filters() {
        let join_memo = HashMap::from([(
            JOIN_GROUP_ID,
            MemoGroupInfo::new(
                vec![
                    Attribute::new_non_null_int64(TEST_ATTR1_NAME.to_string()),
                    Attribute::new_non_null_int64(TEST_ATTR2_NAME.to_string()),
                ]
                .into(),
                GroupAttrRefs::new(
                    vec![
                        AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0),
                        AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0),
                    ],
                    None,
                ),
            ),
        )]);
        let cost_model = create_two_table_mock_cost_model(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            Some(join_memo),
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        let neq12 = bin_op(BinOpType::Neq, attr_index(0), cnst(Value::Int32(12)));
        let eq100 = bin_op(BinOpType::Eq, attr_index(1), cnst(Value::Int32(100)));
        let expr_tree = log_op(LogOpType::And, vec![neq12.clone(), eq100.clone()]);
        let expr_tree_rev = log_op(LogOpType::And, vec![eq100.clone(), neq12.clone()]);

        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree,
                &attr_refs,
                None,
            )
            .await,
            0.2
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev,
                &attr_refs,
                None
            )
            .await,
            0.2
        );
    }

    #[tokio::test]
    async fn test_inner_colref_eq_colref_same_table_is_not_oncond() {
        let cost_model = create_two_table_mock_cost_model(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            None,
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        let expr_tree = bin_op(BinOpType::Eq, attr_index(0), attr_index(0));

        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree,
                &attr_refs,
                None
            )
            .await,
            DEFAULT_EQ_SEL
        );
    }

    // We don't test joinsel or with oncond because if there is an oncond (on condition), the
    // top-level operator must be an AND

    /// I made this helper function to avoid copying all eight lines over and over
    async fn assert_outer_selectivities(
        cost_model: &TestOptCostModelMock,
        expr_tree: ArcPredicateNode,
        expr_tree_rev: ArcPredicateNode,
        attr_refs: &AttrRefs,
        expected_table1_outer_sel: f64,
        expected_table2_outer_sel: f64,
    ) {
        // all table 1 outer combinations
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                false,
                JoinType::LeftOuter,
                expr_tree.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table1_outer_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                false,
                JoinType::LeftOuter,
                expr_tree_rev.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table1_outer_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                true,
                JoinType::RightOuter,
                expr_tree.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table1_outer_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                true,
                JoinType::RightOuter,
                expr_tree_rev.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table1_outer_sel
        );
        // all table 2 outer combinations
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                true,
                JoinType::LeftOuter,
                expr_tree.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table2_outer_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                true,
                JoinType::LeftOuter,
                expr_tree_rev.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table2_outer_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                false,
                JoinType::RightOuter,
                expr_tree.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table2_outer_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                cost_model,
                false,
                JoinType::RightOuter,
                expr_tree_rev.clone(),
                attr_refs,
                None
            )
            .await,
            expected_table2_outer_sel
        );
    }

    /// Unique oncond means an oncondition on columns which are unique in both tables
    /// There's only one case if both columns are unique and have different row counts: the inner
    /// will be < 1 / row count   of one table and = 1 / row count of another
    #[tokio::test]
    async fn test_outer_unique_oncond() {
        let cost_model = create_two_table_mock_cost_model_custom_row_cnts(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            5,
            4,
            None,
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        // the left/right of the join refers to the tables, not the order of columns in the
        // predicate
        let expr_tree = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let expr_tree_rev = bin_op(BinOpType::Eq, attr_index(1), attr_index(0));

        // sanity check the expected inner sel
        let expected_inner_sel = 0.2;
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        // check the outer sels
        assert_outer_selectivities(&cost_model, expr_tree, expr_tree_rev, &attr_refs, 0.25, 0.2);
    }

    /// Non-unique oncond means the column is not unique in either table
    /// Inner always >= row count means that the inner join result is >= 1 / the row count of both
    /// tables
    #[tokio::test]
    async fn test_outer_nonunique_oncond_inner_always_geq_rowcnt() {
        let cost_model = create_two_table_mock_cost_model_custom_row_cnts(
            per_attr_stats_with_ndistinct(5),
            per_attr_stats_with_ndistinct(4),
            10,
            8,
            None,
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        // the left/right of the join refers to the tables, not the order of columns in the
        // predicate
        let expr_tree = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let expr_tree_rev = bin_op(BinOpType::Eq, attr_index(1), attr_index(0));

        // sanity check the expected inner sel
        let expected_inner_sel = 0.2;
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        // check the outer sels
        assert_outer_selectivities(&cost_model, expr_tree, expr_tree_rev, &attr_refs, 0.2, 0.2)
            .await;
    }

    /// Non-unique oncond means the column is not unique in either table
    /// Inner sometimes < row count means that the inner join result < 1 / the row count of exactly
    /// one table.   Note that without a join filter, it's impossible to be less than the row
    /// count of both tables
    #[tokio::test]
    async fn test_outer_nonunique_oncond_inner_sometimes_lt_rowcnt() {
        let cost_model = create_two_table_mock_cost_model_custom_row_cnts(
            per_attr_stats_with_ndistinct(10),
            per_attr_stats_with_ndistinct(2),
            20,
            4,
            None,
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        // the left/right of the join refers to the tables, not the order of columns in the
        // predicate
        let expr_tree = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let expr_tree_rev = bin_op(BinOpType::Eq, attr_index(1), attr_index(0));

        // sanity check the expected inner sel
        let expected_inner_sel = 0.1;
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_rev.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        // check the outer sels
        assert_outer_selectivities(&cost_model, expr_tree, expr_tree_rev, &attr_refs, 0.25, 0.1)
            .await;
    }

    /// Unique oncond means an oncondition on columns which are unique in both tables
    /// Filter means we're adding a join filter
    /// There's only one case if both columns are unique and there's a filter:
    /// the inner will be < 1 / row count of both tables
    #[tokio::test]
    async fn test_outer_unique_oncond_filter() {
        let join_memo = HashMap::from([(
            JOIN_GROUP_ID,
            MemoGroupInfo::new(
                vec![
                    Attribute::new_non_null_int64(TEST_ATTR1_NAME.to_string()),
                    Attribute::new_non_null_int64(TEST_ATTR2_NAME.to_string()),
                ]
                .into(),
                GroupAttrRefs::new(
                    vec![
                        AttrRef::new_base_table_attr_ref(TEST_TABLE1_ID, 0),
                        AttrRef::new_base_table_attr_ref(TEST_TABLE2_ID, 0),
                    ],
                    None,
                ),
            ),
        )]);
        let cost_model = create_two_table_mock_cost_model_custom_row_cnts(
            per_attr_stats_with_dist_and_ndistinct(vec![(Value::Int32(128), 0.4)], 50),
            per_attr_stats_with_ndistinct(4),
            50,
            4,
            Some(join_memo),
        );

        let attr_refs = vec![
            AttrRef::base_table_attr_ref(TEST_TABLE1_ID, 0),
            AttrRef::base_table_attr_ref(TEST_TABLE2_ID, 0),
        ];
        // the left/right of the join refers to the tables, not the order of columns in the
        // predicate
        let eq0and1 = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let eq1and0 = bin_op(BinOpType::Eq, attr_index(1), attr_index(0));
        let filter = bin_op(BinOpType::Leq, attr_index(0), cnst(Value::Int32(128)));
        let expr_tree = log_op(LogOpType::And, vec![eq0and1, filter.clone()]);
        // inner rev means its the inner expr (the eq op) whose children are being reversed, as
        // opposed to the and op
        let expr_tree_inner_rev = log_op(LogOpType::And, vec![eq1and0, filter.clone()]);

        // sanity check the expected inner sel
        let expected_inner_sel = 0.008;
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        assert_approx_eq::assert_approx_eq!(
            test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                expr_tree_inner_rev.clone(),
                &attr_refs,
                None
            )
            .await,
            expected_inner_sel
        );
        // check the outer sels
        assert_outer_selectivities(
            &cost_model,
            expr_tree,
            expr_tree_inner_rev,
            &attr_refs,
            0.25,
            0.02,
        )
        .await;
    }

    /// Test all possible permutations of three-table joins.
    /// A three-table join consists of at least two joins. `join1_on_cond` is the condition of the
    /// first join. There can only be one condition because only two tables are involved at
    /// the time of the first join.
    #[tokio::test]
    #[test_case::test_case(&[(0, 1)])]
    #[test_case::test_case(&[(0, 2)])]
    #[test_case::test_case(&[(1, 2)])]
    #[test_case::test_case(&[(0, 1), (0, 2)])]
    #[test_case::test_case(&[(0, 1), (1, 2)])]
    #[test_case::test_case(&[(0, 2), (1, 2)])]
    #[test_case::test_case(&[(0, 1), (0, 2), (1, 2)])]
    async fn test_three_table_join_for_initial_join_on_conds(
        initial_join_on_conds: &[(usize, usize)],
    ) {
        assert!(
            !initial_join_on_conds.is_empty(),
            "initial_join_on_conds should be non-empty"
        );
        assert_eq!(
            initial_join_on_conds.len(),
            initial_join_on_conds.iter().collect::<HashSet<_>>().len(),
            "initial_join_on_conds shouldn't contain duplicates"
        );
        let cost_model = create_three_table_mock_cost_model(
            per_attr_stats_with_ndistinct(2),
            per_attr_stats_with_ndistinct(3),
            per_attr_stats_with_ndistinct(4),
        );

        let base_attr_refs = vec![
            BaseTableAttrRef {
                table_id: TEST_TABLE1_ID,
                attr_idx: 0,
            },
            BaseTableAttrRef {
                table_id: TEST_TABLE2_ID,
                attr_idx: 0,
            },
            BaseTableAttrRef {
                table_id: TEST_TABLE3_ID,
                attr_idx: 0,
            },
        ];
        let attr_refs = base_attr_refs
            .clone()
            .into_iter()
            .map(AttrRef::BaseTableAttrRef)
            .collect();

        let mut eq_columns = SemanticCorrelation::new();
        for initial_join_on_cond in initial_join_on_conds {
            eq_columns.add_predicate(EqPredicate::new(
                base_attr_refs[initial_join_on_cond.0].clone(),
                base_attr_refs[initial_join_on_cond.1].clone(),
            ));
        }
        let initial_selectivity = {
            if initial_join_on_conds.len() == 1 {
                let initial_join_on_cond = initial_join_on_conds.first().unwrap();
                if initial_join_on_cond == &(0, 1) {
                    1.0 / 3.0
                } else if initial_join_on_cond == &(0, 2) || initial_join_on_cond == &(1, 2) {
                    1.0 / 4.0
                } else {
                    panic!();
                }
            } else {
                1.0 / 12.0
            }
        };

        let input_correlation = Some(eq_columns);

        // Try all join conditions of the final join which would lead to all three tables being
        // joined.
        let eq0and1 = bin_op(BinOpType::Eq, attr_index(0), attr_index(1));
        let eq0and2 = bin_op(BinOpType::Eq, attr_index(0), attr_index(2));
        let eq1and2 = bin_op(BinOpType::Eq, attr_index(1), attr_index(2));
        let and_01_02 = log_op(LogOpType::And, vec![eq0and1.clone(), eq0and2.clone()]);
        let and_01_12 = log_op(LogOpType::And, vec![eq0and1.clone(), eq1and2.clone()]);
        let and_02_12 = log_op(LogOpType::And, vec![eq0and2.clone(), eq1and2.clone()]);
        let and_01_02_12 = log_op(
            LogOpType::And,
            vec![eq0and1.clone(), eq0and2.clone(), eq1and2.clone()],
        );
        let mut join2_expr_trees = vec![and_01_02, and_01_12, and_02_12, and_01_02_12];
        if initial_join_on_conds.len() == 1 {
            let initial_join_on_cond = initial_join_on_conds.first().unwrap();
            if initial_join_on_cond == &(0, 1) {
                join2_expr_trees.push(eq0and2);
                join2_expr_trees.push(eq1and2);
            } else if initial_join_on_cond == &(0, 2) {
                join2_expr_trees.push(eq0and1);
                join2_expr_trees.push(eq1and2);
            } else if initial_join_on_cond == &(1, 2) {
                join2_expr_trees.push(eq0and1);
                join2_expr_trees.push(eq0and2);
            } else {
                panic!();
            }
        }
        for expr_tree in join2_expr_trees {
            let overall_selectivity = initial_selectivity
                * test_get_join_selectivity(
                    &cost_model,
                    false,
                    JoinType::Inner,
                    expr_tree.clone(),
                    &attr_refs,
                    input_correlation.clone(),
                )
                .await;
            assert_approx_eq::assert_approx_eq!(overall_selectivity, 1.0 / 12.0);
        }
    }

    #[tokio::test]
    async fn test_join_which_connects_two_components_together() {
        let cost_model = create_four_table_mock_cost_model(
            per_attr_stats_with_ndistinct(2),
            per_attr_stats_with_ndistinct(3),
            per_attr_stats_with_ndistinct(4),
            per_attr_stats_with_ndistinct(5),
        );
        let base_attr_refs = vec![
            BaseTableAttrRef {
                table_id: TEST_TABLE1_ID,
                attr_idx: 0,
            },
            BaseTableAttrRef {
                table_id: TEST_TABLE2_ID,
                attr_idx: 0,
            },
            BaseTableAttrRef {
                table_id: TEST_TABLE3_ID,
                attr_idx: 0,
            },
            BaseTableAttrRef {
                table_id: TEST_TABLE4_ID,
                attr_idx: 0,
            },
        ];
        let attr_refs = base_attr_refs
            .clone()
            .into_iter()
            .map(AttrRef::BaseTableAttrRef)
            .collect();

        let mut eq_columns = SemanticCorrelation::new();
        eq_columns.add_predicate(EqPredicate::new(
            base_attr_refs[0].clone(),
            base_attr_refs[1].clone(),
        ));
        eq_columns.add_predicate(EqPredicate::new(
            base_attr_refs[2].clone(),
            base_attr_refs[3].clone(),
        ));
        let initial_selectivity = 1.0 / (3.0 * 5.0);
        let input_correlation = Some(eq_columns);

        let eq1and2 = bin_op(BinOpType::Eq, attr_index(1), attr_index(2));
        let overall_selectivity = initial_selectivity
            * test_get_join_selectivity(
                &cost_model,
                false,
                JoinType::Inner,
                eq1and2.clone(),
                &attr_refs,
                input_correlation,
            )
            .await;
        assert_approx_eq::assert_approx_eq!(overall_selectivity, 1.0 / (3.0 * 4.0 * 5.0));
    }
}
