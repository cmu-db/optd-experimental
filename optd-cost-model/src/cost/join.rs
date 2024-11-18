use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    common::{
        nodes::{ArcPredicateNode, JoinType, PredicateType, ReprPredicateNode},
        predicates::{
            attr_ref_pred::AttrRefPred,
            bin_op_pred::BinOpType,
            list_pred::ListPred,
            log_op_pred::{LogOpPred, LogOpType},
        },
        properties::{
            attr_ref::{
                self, AttrRef, AttrRefs, BaseTableAttrRef, EqPredicate, GroupAttrRefs,
                SemanticCorrelation,
            },
            schema::Schema,
        },
    },
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_nlj_row_cnt(
        &self,
        join_typ: JoinType,
        left_row_cnt: f64,
        right_row_cnt: f64,
        output_schema: Schema,
        output_column_refs: GroupAttrRefs,
        join_cond: ArcPredicateNode,
        left_column_refs: GroupAttrRefs,
        right_column_refs: GroupAttrRefs,
    ) -> CostModelResult<f64> {
        let selectivity = {
            let input_correlation = self.get_input_correlation(left_column_refs, right_column_refs);
            self.get_join_selectivity_from_expr_tree(
                join_typ,
                join_cond,
                &output_schema,
                output_column_refs.base_table_attr_refs(),
                input_correlation,
                left_row_cnt,
                right_row_cnt,
            )
            .await?
        };
        Ok((left_row_cnt * right_row_cnt * selectivity).max(1.0))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_hash_join_row_cnt(
        &self,
        join_typ: JoinType,
        left_row_cnt: f64,
        right_row_cnt: f64,
        left_keys: ListPred,
        right_keys: ListPred,
        output_schema: Schema,
        output_column_refs: GroupAttrRefs,
        left_column_refs: GroupAttrRefs,
        right_column_refs: GroupAttrRefs,
    ) -> CostModelResult<f64> {
        let selectivity = {
            let schema = output_schema;
            let column_refs = output_column_refs;
            let column_refs = column_refs.base_table_attr_refs();
            let left_col_cnt = left_column_refs.base_table_attr_refs().len();
            // there may be more than one expression tree in a group.
            // see comment in PredicateType::PhysicalFilter(_) for more information
            let input_correlation = self.get_input_correlation(left_column_refs, right_column_refs);
            self.get_join_selectivity_from_keys(
                join_typ,
                left_keys,
                right_keys,
                &schema,
                column_refs,
                input_correlation,
                left_row_cnt,
                right_row_cnt,
                left_col_cnt,
            )
            .await?
        };
        Ok((left_row_cnt * right_row_cnt * selectivity).max(1.0))
    }

    fn get_input_correlation(
        &self,
        left_prop: GroupAttrRefs,
        right_prop: GroupAttrRefs,
    ) -> Option<SemanticCorrelation> {
        SemanticCorrelation::merge(
            left_prop.output_correlation().cloned(),
            right_prop.output_correlation().cloned(),
        )
    }

    /// A wrapper to convert the join keys to the format expected by get_join_selectivity_core()
    #[allow(clippy::too_many_arguments)]
    async fn get_join_selectivity_from_keys(
        &self,
        join_typ: JoinType,
        left_keys: ListPred,
        right_keys: ListPred,
        schema: &Schema,
        column_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        left_row_cnt: f64,
        right_row_cnt: f64,
        left_col_cnt: usize,
    ) -> CostModelResult<f64> {
        assert!(left_keys.len() == right_keys.len());
        // I assume that the keys are already in the right order
        // s.t. the ith key of left_keys corresponds with the ith key of right_keys
        let on_col_ref_pairs = left_keys
            .to_vec()
            .into_iter()
            .zip(right_keys.to_vec())
            .map(|(left_key, right_key)| {
                (
                    AttrRefPred::from_pred_node(left_key).expect("keys should be AttrRefPreds"),
                    AttrRefPred::from_pred_node(right_key).expect("keys should be AttrRefPreds"),
                )
            })
            .collect_vec();
        self.get_join_selectivity_core(
            join_typ,
            on_col_ref_pairs,
            None,
            schema,
            column_refs,
            input_correlation,
            left_row_cnt,
            right_row_cnt,
            left_col_cnt,
        )
        .await
    }

    /// The core logic of join selectivity which assumes we've already separated the expression
    /// into the on conditions and the filters.
    ///
    /// Hash join and NLJ reference right table columns differently, hence the
    /// `right_col_ref_offset` parameter.
    ///
    /// For hash join, the right table columns indices are with respect to the right table,
    /// which means #0 is the first column of the right table.
    ///
    /// For NLJ, the right table columns indices are with respect to the output of the join.
    /// For example, if the left table has 3 columns, the first column of the right table
    /// is #3 instead of #0.
    #[allow(clippy::too_many_arguments)]
    async fn get_join_selectivity_core(
        &self,
        join_typ: JoinType,
        on_col_ref_pairs: Vec<(AttrRefPred, AttrRefPred)>,
        filter_expr_tree: Option<ArcPredicateNode>,
        schema: &Schema,
        column_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        left_row_cnt: f64,
        right_row_cnt: f64,
        right_col_ref_offset: usize,
    ) -> CostModelResult<f64> {
        let join_on_selectivity = self
            .get_join_on_selectivity(
                &on_col_ref_pairs,
                column_refs,
                input_correlation,
                right_col_ref_offset,
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
                // FIXME: Pass in group id or schema & attr_refs
                self.get_filter_selectivity(filter_expr_tree).await?
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
                    on_col_ref_pairs.is_empty(),
                    "Cross joins should not have on columns"
                );
                join_filter_selectivity
            }
            _ => unimplemented!("join_typ={} is not implemented", join_typ),
        })
    }

    /// The expr_tree input must be a "mixed expression tree", just like with
    /// `get_filter_selectivity`.
    ///
    /// This is a "wrapper" to separate the equality conditions from the filter conditions before
    /// calling the "main" `get_join_selectivity_core` function.
    #[allow(clippy::too_many_arguments)]
    async fn get_join_selectivity_from_expr_tree(
        &self,
        join_typ: JoinType,
        expr_tree: ArcPredicateNode,
        schema: &Schema,
        column_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        left_row_cnt: f64,
        right_row_cnt: f64,
    ) -> CostModelResult<f64> {
        if expr_tree.typ == PredicateType::LogOp(LogOpType::And) {
            let mut on_col_ref_pairs = vec![];
            let mut filter_expr_trees = vec![];
            for child_expr_tree in &expr_tree.children {
                if let Some(on_col_ref_pair) =
                    Self::get_on_col_ref_pair(child_expr_tree.clone(), column_refs)
                {
                    on_col_ref_pairs.push(on_col_ref_pair)
                } else {
                    let child_expr = child_expr_tree.clone();
                    filter_expr_trees.push(child_expr);
                }
            }
            assert!(on_col_ref_pairs.len() + filter_expr_trees.len() == expr_tree.children.len());
            let filter_expr_tree = if filter_expr_trees.is_empty() {
                None
            } else {
                Some(LogOpPred::new(LogOpType::And, filter_expr_trees).into_pred_node())
            };
            self.get_join_selectivity_core(
                join_typ,
                on_col_ref_pairs,
                filter_expr_tree,
                schema,
                column_refs,
                input_correlation,
                left_row_cnt,
                right_row_cnt,
                0,
            )
            .await
        } else {
            #[allow(clippy::collapsible_else_if)]
            if let Some(on_col_ref_pair) = Self::get_on_col_ref_pair(expr_tree.clone(), column_refs)
            {
                self.get_join_selectivity_core(
                    join_typ,
                    vec![on_col_ref_pair],
                    None,
                    schema,
                    column_refs,
                    input_correlation,
                    left_row_cnt,
                    right_row_cnt,
                    0,
                )
                .await
            } else {
                self.get_join_selectivity_core(
                    join_typ,
                    vec![],
                    Some(expr_tree),
                    schema,
                    column_refs,
                    input_correlation,
                    left_row_cnt,
                    right_row_cnt,
                    0,
                )
                .await
            }
        }
    }

    /// Check if an expr_tree is a join condition, returning the join on col ref pair if it is.
    /// The reason the check and the info are in the same function is because their code is almost
    /// identical. It only picks out equality conditions between two column refs on different
    /// tables
    fn get_on_col_ref_pair(
        expr_tree: ArcPredicateNode,
        column_refs: &AttrRefs,
    ) -> Option<(AttrRefPred, AttrRefPred)> {
        // 1. Check that it's equality
        if expr_tree.typ == PredicateType::BinOp(BinOpType::Eq) {
            let left_child = expr_tree.child(0);
            let right_child = expr_tree.child(1);
            // 2. Check that both sides are column refs
            if left_child.typ == PredicateType::AttrRef && right_child.typ == PredicateType::AttrRef
            {
                // 3. Check that both sides don't belong to the same table (if we don't know, that
                //    means they don't belong)
                let left_col_ref_expr = AttrRefPred::from_pred_node(left_child)
                    .expect("we already checked that the type is AttrRef");
                let right_col_ref_expr = AttrRefPred::from_pred_node(right_child)
                    .expect("we already checked that the type is AttrRef");
                let left_col_ref = &column_refs[left_col_ref_expr.attr_index() as usize];
                let right_col_ref = &column_refs[right_col_ref_expr.attr_index() as usize];
                let is_same_table = if let (
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: left_table_id,
                        ..
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: right_table_id,
                        ..
                    }),
                ) = (left_col_ref, right_col_ref)
                {
                    left_table_id == right_table_id
                } else {
                    false
                };
                if !is_same_table {
                    Some((left_col_ref_expr, right_col_ref_expr))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the selectivity of one column eq predicate, e.g. colA = colB.
    async fn get_join_selectivity_from_on_col_ref_pair(
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
                        Some(per_col_stats) => per_col_stats.ndistinct,
                        None => DEFAULT_NUM_DISTINCT,
                    }
                }
                AttrRef::Derived => DEFAULT_NUM_DISTINCT,
            };
            ndistincts.push(ndistinct);
        }

        // using reduce(f64::min) is the idiomatic workaround to min() because
        // f64 does not implement Ord due to NaN
        let selectivity = ndistincts.into_iter().map(|ndistinct| 1.0 / ndistinct as f64).reduce(f64::min).expect("reduce() only returns None if the iterator is empty, which is impossible since col_ref_exprs.len() == 2");
        assert!(
            !selectivity.is_nan(),
            "it should be impossible for selectivity to be NaN since n-distinct is never 0"
        );
        Ok(selectivity)
    }

    /// Given a set of N columns involved in a multi-equality, find the total selectivity
    /// of the multi-equality.
    ///
    /// This is a generalization of get_join_selectivity_from_on_col_ref_pair().
    async fn get_join_selectivity_from_most_selective_columns(
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
                Some(per_col_stats) => per_col_stats.ndistinct,
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
    /// The nodes are columns while edges are predicates. The old graph is defined by
    /// `past_eq_columns` while the `predicate` is the new addition to this graph. This
    /// unweighted undirected graph consists of a number of connected components, where each
    /// connected component represents columns that are set to be equal to each other. Single
    /// nodes not connected to anything are considered standalone connected components.
    ///
    /// The selectivity of each connected component of N nodes is equal to the product of
    /// 1/ndistinct of the N-1 nodes with the highest ndistinct values. You can see this if you
    /// imagine that all columns being joined are unique columns and that they follow the
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
    /// NOTE: This function modifies `past_eq_columns` by adding `predicate` to it.
    async fn get_join_selectivity_adjustment_when_adding_to_multi_equality_graph(
        &self,
        predicate: &EqPredicate,
        past_eq_columns: &mut SemanticCorrelation,
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
        // columns not involved in any predicates are considered a part of the graph and are
        // a connected component on their own.
        let children_pred_sel = {
            if past_eq_columns.is_eq(&predicate.left, &predicate.right) {
                self.get_join_selectivity_from_most_selective_columns(
                    past_eq_columns.find_attrs_for_eq_attribute_set(&predicate.left),
                )
                .await?
            } else {
                let left_sel = if past_eq_columns.contains(&predicate.left) {
                    self.get_join_selectivity_from_most_selective_columns(
                        past_eq_columns.find_attrs_for_eq_attribute_set(&predicate.left),
                    )
                    .await?
                } else {
                    1.0
                };
                let right_sel = if past_eq_columns.contains(&predicate.right) {
                    self.get_join_selectivity_from_most_selective_columns(
                        past_eq_columns.find_attrs_for_eq_attribute_set(&predicate.right),
                    )
                    .await?
                } else {
                    1.0
                };
                left_sel * right_sel
            }
        };

        // Add predicate to past_eq_columns and compute the selectivity of the connected component
        // it creates.
        past_eq_columns.add_predicate(predicate.clone());
        let new_pred_sel = {
            let cols = past_eq_columns.find_attrs_for_eq_attribute_set(&predicate.left);
            self.get_join_selectivity_from_most_selective_columns(cols)
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
        on_col_ref_pairs: &[(AttrRefPred, AttrRefPred)],
        column_refs: &AttrRefs,
        input_correlation: Option<SemanticCorrelation>,
        right_col_ref_offset: usize,
    ) -> CostModelResult<f64> {
        let mut past_eq_columns = input_correlation.unwrap_or_default();

        // Multiply the selectivities of all individual conditions together
        let mut selectivity = 1.0;
        for on_col_ref_pair in on_col_ref_pairs {
            let left_col_ref = &column_refs[on_col_ref_pair.0.attr_index() as usize];
            let right_col_ref =
                &column_refs[on_col_ref_pair.1.attr_index() as usize + right_col_ref_offset];

            if let (AttrRef::BaseTableAttrRef(left), AttrRef::BaseTableAttrRef(right)) =
                (left_col_ref, right_col_ref)
            {
                let predicate = EqPredicate::new(left.clone(), right.clone());
                return self
                    .get_join_selectivity_adjustment_when_adding_to_multi_equality_graph(
                        &predicate,
                        &mut past_eq_columns,
                    )
                    .await;
            }

            selectivity *= self
                .get_join_selectivity_from_on_col_ref_pair(left_col_ref, right_col_ref)
                .await?;
        }
        Ok(selectivity)
    }
}
