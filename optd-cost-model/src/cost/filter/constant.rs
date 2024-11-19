use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType},
        predicates::constant_pred::ConstantType,
        values::Value,
    },
    cost_model::CostModelImpl,
    storage::CostModelStorageManager,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    pub(crate) fn get_constant_selectivity(const_node: ArcPredicateNode) -> f64 {
        if let PredicateType::Constant(const_typ) = const_node.typ {
            if matches!(const_typ, ConstantType::Bool) {
                let value = const_node
                    .as_ref()
                    .data
                    .as_ref()
                    .expect("constants should have data");
                if let Value::Bool(bool_value) = value {
                    if *bool_value {
                        1.0
                    } else {
                        0.0
                    }
                } else {
                    unreachable!(
                        "if the typ is ConstantType::Bool, the value should be a Value::Bool"
                    )
                }
            } else {
                panic!("selectivity is not defined on constants which are not bools")
            }
        } else {
            panic!("get_constant_selectivity must be called on a constant")
        }
    }
}
