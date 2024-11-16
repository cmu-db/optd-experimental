use crate::{
    common::{
        nodes::{PredicateType, ReprPredicateNode},
        predicates::{
            attr_ref_pred::AttributeRefPred, constant_pred::ConstantPred, in_list_pred::InListPred,
        },
    },
    cost_model::CostModelImpl,
    stats::UNIMPLEMENTED_SEL,
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// Only support attrA in (val1, val2, val3) where attrA is a attribute ref and
    /// val1, val2, val3 are constants.
    pub(crate) async fn get_in_list_selectivity(&self, expr: &InListPred) -> CostModelResult<f64> {
        let child = expr.child();

        // Check child is a attribute ref.
        if !matches!(child.typ, PredicateType::AttributeRef) {
            return Ok(UNIMPLEMENTED_SEL);
        }

        // Check all expressions in the list are constants.
        let list_exprs = expr.list().to_vec();
        if list_exprs
            .iter()
            .any(|expr| !matches!(expr.typ, PredicateType::Constant(_)))
        {
            return Ok(UNIMPLEMENTED_SEL);
        }

        // Convert child and const expressions to concrete types.
        let attr_ref_pred = AttributeRefPred::from_pred_node(child).unwrap();
        let attr_ref_idx = attr_ref_pred.attr_index();
        let table_id = attr_ref_pred.table_id();
        let list_exprs = list_exprs
            .into_iter()
            .map(|expr| {
                ConstantPred::from_pred_node(expr)
                    .expect("we already checked all list elements are constants")
            })
            .collect::<Vec<_>>();
        let negated = expr.negated();

        // TODO: Consider attribute is a derived attribute
        let mut in_sel = 0.0;
        for expr in &list_exprs {
            let selectivity = self
                .get_attribute_equality_selectivity(
                    table_id,
                    attr_ref_idx,
                    &expr.value(),
                    /* is_equality */ true,
                )
                .await?;
            in_sel += selectivity;
        }
        in_sel = in_sel.min(1.0);
        if negated {
            Ok(1.0 - in_sel)
        } else {
            Ok(in_sel)
        }
    }
}
