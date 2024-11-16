use optd_persistent::CostModelStorageLayer;

use crate::{
    common::{nodes::ArcPredicateNode, predicates::log_op_pred::LogOpType},
    cost_model::CostModelImpl,
    CostModelResult,
};

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    pub(crate) async fn get_log_op_selectivity(
        &self,
        log_op_typ: LogOpType,
        children: &[ArcPredicateNode],
    ) -> CostModelResult<f64> {
        match log_op_typ {
            LogOpType::And => {
                let mut and_sel = 1.0;
                for child in children {
                    let selectivity = self.get_filter_selectivity(child.clone()).await?;
                    and_sel *= selectivity;
                }
                Ok(and_sel)
            }
            LogOpType::Or => {
                let mut or_sel_neg = 1.0;
                for child in children {
                    let selectivity = self.get_filter_selectivity(child.clone()).await?;
                    or_sel_neg *= (1.0 - selectivity);
                }
                Ok(1.0 - or_sel_neg)
            }
        }
    }
}
