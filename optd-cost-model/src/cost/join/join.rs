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
        properties::attr_ref::{
            self, AttrRef, AttrRefs, BaseTableAttrRef, EqPredicate, GroupAttrRefs,
            SemanticCorrelation,
        },
        types::GroupId,
    },
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    storage::CostModelStorageManager,
    CostModelResult,
};

pub(crate) fn get_input_correlation(
    left_prop: GroupAttrRefs,
    right_prop: GroupAttrRefs,
) -> Option<SemanticCorrelation> {
    SemanticCorrelation::merge(
        left_prop.output_correlation().cloned(),
        right_prop.output_correlation().cloned(),
    )
}

/// Check if an expr_tree is a join condition, returning the join on attr ref pair if it is.
/// The reason the check and the info are in the same function is because their code is almost
/// identical. It only picks out equality conditions between two attribute refs on different
/// tables
pub(crate) fn get_on_attr_ref_pair(
    expr_tree: ArcPredicateNode,
    attr_refs: &AttrRefs,
) -> Option<(AttrRefPred, AttrRefPred)> {
    // 1. Check that it's equality
    if expr_tree.typ == PredicateType::BinOp(BinOpType::Eq) {
        let left_child = expr_tree.child(0);
        let right_child = expr_tree.child(1);
        // 2. Check that both sides are attribute refs
        if left_child.typ == PredicateType::AttrRef && right_child.typ == PredicateType::AttrRef {
            // 3. Check that both sides don't belong to the same table (if we don't know, that
            //    means they don't belong)
            let left_attr_ref_expr = AttrRefPred::from_pred_node(left_child)
                .expect("we already checked that the type is AttrRef");
            let right_attr_ref_expr = AttrRefPred::from_pred_node(right_child)
                .expect("we already checked that the type is AttrRef");
            let left_attr_ref = &attr_refs[left_attr_ref_expr.attr_index() as usize];
            let right_attr_ref = &attr_refs[right_attr_ref_expr.attr_index() as usize];
            let is_same_table = if let (
                AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                    table_id: left_table_id,
                    ..
                }),
                AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                    table_id: right_table_id,
                    ..
                }),
            ) = (left_attr_ref, right_attr_ref)
            {
                left_table_id == right_table_id
            } else {
                false
            };
            if !is_same_table {
                Some((left_attr_ref_expr, right_attr_ref_expr))
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

impl<S: CostModelStorageManager> CostModelImpl<S> {
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
    pub(crate) async fn get_join_selectivity_core(
        &self,
        join_typ: JoinType,
        on_attr_ref_pairs: Vec<(AttrRefPred, AttrRefPred)>,
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
        on_attr_ref_pairs: &[(AttrRefPred, AttrRefPred)],
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

            if let (AttrRef::BaseTableAttrRef(left), AttrRef::BaseTableAttrRef(right)) =
                (left_attr_ref, right_attr_ref)
            {
                let predicate = EqPredicate::new(left.clone(), right.clone());
                return self
                    .get_join_selectivity_adjustment_when_adding_to_multi_equality_graph(
                        &predicate,
                        &mut past_eq_attrs,
                    )
                    .await;
            }

            selectivity *= self
                .get_join_selectivity_from_on_attr_ref_pair(left_attr_ref, right_attr_ref)
                .await?;
        }
        Ok(selectivity)
    }
}
