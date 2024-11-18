use itertools::Itertools;

use crate::{
    common::{
        nodes::{JoinType, ReprPredicateNode},
        predicates::{attr_ref_pred::AttrRefPred, list_pred::ListPred},
        properties::attr_ref::{AttrRefs, SemanticCorrelation},
        types::GroupId,
    },
    cost_model::CostModelImpl,
    storage::CostModelStorageManager,
    CostModelResult, EstimatedStatistic,
};

use super::join::get_input_correlation;

impl<S: CostModelStorageManager> CostModelImpl<S> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_hash_join_row_cnt(
        &self,
        join_typ: JoinType,
        group_id: GroupId,
        left_row_cnt: f64,
        right_row_cnt: f64,
        left_group_id: GroupId,
        right_group_id: GroupId,
        left_keys: ListPred,
        right_keys: ListPred,
    ) -> CostModelResult<EstimatedStatistic> {
        let selectivity = {
            let output_attr_refs = self.memo.get_attribute_ref(group_id);
            let left_attr_refs = self.memo.get_attribute_ref(left_group_id);
            let right_attr_refs = self.memo.get_attribute_ref(right_group_id);
            let left_attr_cnt = left_attr_refs.attr_refs().len();
            // there may be more than one expression tree in a group.
            // see comment in PredicateType::PhysicalFilter(_) for more information
            let input_correlation = get_input_correlation(left_attr_refs, right_attr_refs);
            self.get_hash_join_selectivity(
                join_typ,
                left_keys,
                right_keys,
                output_attr_refs.attr_refs(),
                input_correlation,
                left_row_cnt,
                right_row_cnt,
                left_attr_cnt,
            )
            .await?
        };
        Ok(EstimatedStatistic(
            (left_row_cnt * right_row_cnt * selectivity).max(1.0),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    async fn get_hash_join_selectivity(
        &self,
        join_typ: JoinType,
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
                    AttrRefPred::from_pred_node(left_key).expect("keys should be AttrRefPreds"),
                    AttrRefPred::from_pred_node(right_key).expect("keys should be AttrRefPreds"),
                )
            })
            .collect_vec();
        self.get_join_selectivity_core(
            join_typ,
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
}
