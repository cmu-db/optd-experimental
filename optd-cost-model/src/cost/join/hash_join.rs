use crate::{
    common::{nodes::JoinType, predicates::list_pred::ListPred, types::GroupId},
    cost_model::CostModelImpl,
    storage::CostModelStorageManager,
    CostModelResult, EstimatedStatistic,
};

use super::get_input_correlation;

impl<S: CostModelStorageManager> CostModelImpl<S> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_hash_join_row_cnt(
        &self,
        join_typ: JoinType,
        group_id: GroupId,
        left_row_cnt: EstimatedStatistic,
        right_row_cnt: EstimatedStatistic,
        left_group_id: GroupId,
        right_group_id: GroupId,
        left_keys: ListPred,
        right_keys: ListPred,
    ) -> CostModelResult<EstimatedStatistic> {
        let selectivity = {
            let output_attr_refs = self.memo.get_attribute_refs(group_id);
            let left_attr_refs = self.memo.get_attribute_refs(left_group_id);
            let right_attr_refs = self.memo.get_attribute_refs(right_group_id);
            let left_attr_cnt = left_attr_refs.attr_refs().len();
            // there may be more than one expression tree in a group.
            // see comment in PredicateType::PhysicalFilter(_) for more information
            let input_correlation = get_input_correlation(left_attr_refs, right_attr_refs);
            self.get_join_selectivity_from_keys(
                join_typ,
                group_id,
                left_keys,
                right_keys,
                output_attr_refs.attr_refs(),
                input_correlation,
                left_row_cnt.0,
                right_row_cnt.0,
                left_attr_cnt,
            )
            .await?
        };
        Ok(EstimatedStatistic(
            (left_row_cnt.0 * right_row_cnt.0 * selectivity).max(1.0),
        ))
    }
}
