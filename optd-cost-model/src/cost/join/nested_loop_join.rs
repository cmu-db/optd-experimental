use crate::{
    common::{
        nodes::{ArcPredicateNode, JoinType, PredicateType, ReprPredicateNode},
        predicates::log_op_pred::{LogOpPred, LogOpType},
        properties::attr_ref::{AttrRefs, SemanticCorrelation},
        types::GroupId,
    },
    cost::join::join::get_on_attr_ref_pair,
    cost_model::CostModelImpl,
    storage::CostModelStorageManager,
    CostModelResult, EstimatedStatistic,
};

use super::join::get_input_correlation;

impl<S: CostModelStorageManager> CostModelImpl<S> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_nlj_row_cnt(
        &self,
        join_typ: JoinType,
        group_id: GroupId,
        left_row_cnt: f64,
        right_row_cnt: f64,
        left_group_id: GroupId,
        right_group_id: GroupId,
        join_cond: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let selectivity = {
            let output_attr_refs = self.memo.get_attribute_ref(group_id);
            let left_attr_refs = self.memo.get_attribute_ref(left_group_id);
            let right_attr_refs = self.memo.get_attribute_ref(right_group_id);
            let input_correlation = get_input_correlation(left_attr_refs, right_attr_refs);

            self.get_nlj_join_selectivity(
                join_typ,
                join_cond,
                output_attr_refs.attr_refs(),
                input_correlation,
                left_row_cnt,
                right_row_cnt,
            )
            .await?
        };
        Ok(EstimatedStatistic(
            (left_row_cnt * right_row_cnt * selectivity).max(1.0),
        ))
    }

    /// The expr_tree input must be a "mixed expression tree", just like with
    /// `get_filter_selectivity`.
    ///
    /// This is a "wrapper" to separate the equality conditions from the filter conditions before
    /// calling the "main" `get_join_selectivity_core` function.
    #[allow(clippy::too_many_arguments)]
    async fn get_nlj_join_selectivity(
        &self,
        join_typ: JoinType,
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
}
