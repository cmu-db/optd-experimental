use std::fmt::Display;

use crate::common::nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode};

/// TODO: documentation
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum UnOpType {
    Neg = 1,
    Not,
}

impl Display for UnOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct UnOpPred(pub ArcPredicateNode);

impl UnOpPred {
    pub fn new(child: ArcPredicateNode, op_type: UnOpType) -> Self {
        UnOpPred(
            PredicateNode {
                typ: PredicateType::UnOp(op_type),
                children: vec![child],
                data: None,
            }
            .into(),
        )
    }

    pub fn child(&self) -> ArcPredicateNode {
        self.0.child(0)
    }

    pub fn op_type(&self) -> UnOpType {
        if let PredicateType::UnOp(op_type) = self.0.typ {
            op_type
        } else {
            panic!("not a un op")
        }
    }
}

impl ReprPredicateNode for UnOpPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if !matches!(pred_node.typ, PredicateType::UnOp(_)) {
            return None;
        }
        Some(Self(pred_node))
    }
}
