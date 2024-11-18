use crate::{
    common::{
        nodes::{ArcPredicateNode, JoinType, PredicateType, ReprPredicateNode},
        predicates::log_op_pred::{LogOpPred, LogOpType},
        properties::attr_ref::{AttrRefs, SemanticCorrelation},
        types::GroupId,
    },
    cost_model::CostModelImpl,
    storage::CostModelStorageManager,
    CostModelResult, EstimatedStatistic,
};

use super::get_input_correlation;

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

            self.get_join_selectivity_from_expr_tree(
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
}