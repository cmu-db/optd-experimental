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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        common::{types::TableId, values::Value},
        cost_model::tests::*,
        stats::{
            utilities::{counter::Counter, simple_map::SimpleMap},
            MostCommonValues,
        },
    };

    #[tokio::test]
    async fn test_in_list() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(1))], 0.8),
                (vec![Some(Value::Int32(2))], 0.2),
            ])),
            2,
            0.0,
            None,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![per_attribute_stats],
            vec![None],
            HashMap::new(),
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(&in_list(table_id, 0, vec![Value::Int32(1)], false))
                .await
                .unwrap(),
            0.8
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(&in_list(
                    table_id,
                    0,
                    vec![Value::Int32(1), Value::Int32(2)],
                    false
                ))
                .await
                .unwrap(),
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(&in_list(table_id, 0, vec![Value::Int32(3)], false))
                .await
                .unwrap(),
            0.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(&in_list(table_id, 0, vec![Value::Int32(1)], true))
                .await
                .unwrap(),
            0.2
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(&in_list(
                    table_id,
                    0,
                    vec![Value::Int32(1), Value::Int32(2)],
                    true
                ))
                .await
                .unwrap(),
            0.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(&in_list(table_id, 0, vec![Value::Int32(3)], true))
                .await
                .unwrap(),
            1.0
        );
    }
}
