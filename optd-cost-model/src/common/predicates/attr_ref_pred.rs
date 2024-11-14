use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    values::Value,
};

#[derive(Clone, Debug)]
pub struct AttributeRefPred(pub ArcPredicateNode);

impl AttributeRefPred {
    /// Creates a new `ColumnRef` expression.
    pub fn new(column_idx: usize) -> AttributeRefPred {
        // this conversion is always safe since usize is at most u64
        let u64_column_idx = column_idx as u64;
        AttributeRefPred(
            PredicateNode {
                typ: PredicateType::AttributeRef,
                children: vec![],
                data: Some(Value::UInt64(u64_column_idx)),
            }
            .into(),
        )
    }

    fn get_data_usize(&self) -> usize {
        self.0.data.as_ref().unwrap().as_u64() as usize
    }

    /// Gets the column index.
    pub fn index(&self) -> usize {
        self.get_data_usize()
    }
}

impl ReprPredicateNode for AttributeRefPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if pred_node.typ != PredicateType::AttributeRef {
            return None;
        }
        Some(Self(pred_node))
    }
}
