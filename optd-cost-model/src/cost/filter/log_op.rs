use crate::{
    common::{nodes::ArcPredicateNode, predicates::log_op_pred::LogOpType, types::GroupId},
    cost_model::CostModelImpl,
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    pub(crate) async fn get_log_op_selectivity(
        &self,
        group_id: GroupId,
        log_op_typ: LogOpType,
        children: &[ArcPredicateNode],
    ) -> CostModelResult<f64> {
        match log_op_typ {
            LogOpType::And => {
                let mut and_sel = 1.0;
                for child in children {
                    let selectivity = self.get_filter_selectivity(group_id, child.clone()).await?;
                    and_sel *= selectivity;
                }
                Ok(and_sel)
            }
            LogOpType::Or => {
                let mut or_sel_neg = 1.0;
                for child in children {
                    let selectivity = self.get_filter_selectivity(group_id, child.clone()).await?;
                    or_sel_neg *= 1.0 - selectivity;
                }
                Ok(1.0 - or_sel_neg)
            }
        }
    }
}
