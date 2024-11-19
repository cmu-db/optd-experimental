use arrow_schema::DataType;

use crate::common::nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode};

#[derive(Clone, Debug)]
pub struct DataTypePred(pub ArcPredicateNode);

impl DataTypePred {
    pub fn new(typ: DataType) -> Self {
        DataTypePred(
            PredicateNode {
                typ: PredicateType::DataType(typ),
                children: vec![],
                data: None,
            }
            .into(),
        )
    }

    pub fn data_type(&self) -> DataType {
        if let PredicateType::DataType(ref data_type) = self.0.typ {
            data_type.clone()
        } else {
            panic!("not a data type")
        }
    }
}

impl ReprPredicateNode for DataTypePred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if !matches!(pred_node.typ, PredicateType::DataType(_)) {
            return None;
        }
        Some(Self(pred_node))
    }
}
