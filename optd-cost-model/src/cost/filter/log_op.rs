use optd_persistent::CostModelStorageLayer;

use crate::{
    common::{nodes::ArcPredicateNode, predicates::log_op_pred::LogOpType},
    cost_model::CostModelImpl,
    CostModelResult,
};

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    pub(crate) fn get_log_op_selectivity(
        &self,
        log_op_typ: LogOpType,
        children: &[ArcPredicateNode],
    ) -> CostModelResult<f64> {
        match log_op_typ {
            LogOpType::And => children.iter().try_fold(1.0, |acc, child| {
                let selectivity = self.get_filter_selectivity(child.clone())?;
                Ok(acc * selectivity)
            }),
            LogOpType::Or => {
                let product = children.iter().try_fold(1.0, |acc, child| {
                    let selectivity = self.get_filter_selectivity(child.clone())?;
                    Ok(acc * (1.0 - selectivity))
                })?;
                Ok(1.0 - product)
            }
        }
    }
}
