use arrow_schema::DataType;

use crate::common::nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode};

use super::data_type_pred::DataTypePred;

/// [`CastPred`] casts a column from one data type to another.
///
/// A [`CastPred`] has two children:
/// 1. The original data to cast
/// 2. The target data type to cast to
#[derive(Clone, Debug)]
pub struct CastPred(pub ArcPredicateNode);

impl CastPred {
    pub fn new(child: ArcPredicateNode, cast_to: DataType) -> Self {
        CastPred(
            PredicateNode {
                typ: PredicateType::Cast,
                children: vec![child, DataTypePred::new(cast_to).into_pred_node()],
                data: None,
            }
            .into(),
        )
    }

    pub fn child(&self) -> ArcPredicateNode {
        self.0.child(0)
    }

    pub fn cast_to(&self) -> DataType {
        DataTypePred::from_pred_node(self.0.child(1))
            .unwrap()
            .data_type()
    }
}

impl ReprPredicateNode for CastPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if !matches!(pred_node.typ, PredicateType::Cast) {
            return None;
        }
        Some(Self(pred_node))
    }
}
