use crate::{
    common::{
        nodes::{PredicateType, ReprPredicateNode},
        predicates::{
            attr_index_pred::AttrIndexPred, constant_pred::ConstantPred, in_list_pred::InListPred,
        },
        properties::attr_ref::{AttrRef, BaseTableAttrRef},
        types::GroupId,
    },
    cost_model::CostModelImpl,
    stats::UNIMPLEMENTED_SEL,
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// Only support attrA in (val1, val2, val3) where attrA is a attribute ref and
    /// val1, val2, val3 are constants.
    pub(crate) async fn get_in_list_selectivity(
        &self,
        group_id: GroupId,
        expr: &InListPred,
    ) -> CostModelResult<f64> {
        let child = expr.child();

        // Check child is a attribute ref.
        if !matches!(child.typ, PredicateType::AttrIndex) {
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
        let attr_ref_pred = AttrIndexPred::from_pred_node(child).unwrap();
        let attr_ref_idx = attr_ref_pred.attr_index();

        let list_exprs = list_exprs
            .into_iter()
            .map(|expr| {
                ConstantPred::from_pred_node(expr)
                    .expect("we already checked all list elements are constants")
            })
            .collect::<Vec<_>>();
        let negated = expr.negated();

        if let AttrRef::BaseTableAttrRef(BaseTableAttrRef { table_id, attr_idx }) =
            self.memo.get_attribute_ref(group_id, attr_ref_idx)
        {
            let mut in_sel = 0.0;
            for expr in &list_exprs {
                let selectivity = self
                    .get_attribute_equality_selectivity(
                        table_id,
                        attr_idx,
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
        } else {
            // TODO: Child is a derived attribute.
            Ok(UNIMPLEMENTED_SEL)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        common::values::Value,
        stats::{utilities::simple_map::SimpleMap, MostCommonValues},
        test_utils::tests::*,
    };

    #[tokio::test]
    async fn test_in_list() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(1))], 0.8),
                (vec![Some(Value::Int32(2))], 0.2),
            ])),
            None,
            2,
            0.0,
        );
        let cost_model = create_mock_cost_model(
            vec![TEST_TABLE1_ID],
            vec![HashMap::from([(
                TEST_ATTR1_BASE_INDEX,
                per_attribute_stats,
            )])],
            vec![None],
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(TEST_GROUP1_ID, &in_list(0, vec![Value::Int32(1)], false))
                .await
                .unwrap(),
            0.8
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(
                    TEST_GROUP1_ID,
                    &in_list(0, vec![Value::Int32(1), Value::Int32(2)], false)
                )
                .await
                .unwrap(),
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(TEST_GROUP1_ID, &in_list(0, vec![Value::Int32(3)], false))
                .await
                .unwrap(),
            0.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(TEST_GROUP1_ID, &in_list(0, vec![Value::Int32(1)], true))
                .await
                .unwrap(),
            0.2
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(
                    TEST_GROUP1_ID,
                    &in_list(0, vec![Value::Int32(1), Value::Int32(2)], true)
                )
                .await
                .unwrap(),
            0.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_in_list_selectivity(TEST_GROUP1_ID, &in_list(0, vec![Value::Int32(3)], true)) // TODO: Fix this
                .await
                .unwrap(),
            1.0
        );
    }
}
