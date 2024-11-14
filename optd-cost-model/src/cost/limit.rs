use optd_persistent::CostModelStorageLayer;

use crate::{
    common::{
        nodes::{ArcPredicateNode, ReprPredicateNode},
        predicates::constant_pred::ConstantPred,
    },
    cost_model::CostModelImpl,
    CostModelResult, EstimatedStatistic,
};

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    pub(crate) fn get_limit_row_cnt(
        &self,
        child_row_cnt: EstimatedStatistic,
        fetch_expr: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let fetch = ConstantPred::from_pred_node(fetch_expr)
            .unwrap()
            .value()
            .as_u64();
        // u64::MAX represents None
        if fetch == u64::MAX {
            Ok(child_row_cnt)
        } else {
            Ok(EstimatedStatistic(child_row_cnt.0.min(fetch)))
        }
    }
}
