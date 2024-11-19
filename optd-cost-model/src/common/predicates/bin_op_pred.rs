use crate::common::nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode};

/// TODO: documentation
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BinOpType {
    // numerical
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // comparison
    Eq,
    Neq,
    Gt,
    Lt,
    Geq,
    Leq,
}

impl std::fmt::Display for BinOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BinOpType {
    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod
        )
    }

    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            Self::Eq | Self::Neq | Self::Gt | Self::Lt | Self::Geq | Self::Leq
        )
    }
}

#[derive(Clone, Debug)]
pub struct BinOpPred(pub ArcPredicateNode);

impl BinOpPred {
    pub fn new(left: ArcPredicateNode, right: ArcPredicateNode, op_type: BinOpType) -> Self {
        BinOpPred(
            PredicateNode {
                typ: PredicateType::BinOp(op_type),
                children: vec![left, right],
                data: None,
            }
            .into(),
        )
    }

    pub fn left_child(&self) -> ArcPredicateNode {
        self.0.child(0)
    }

    pub fn right_child(&self) -> ArcPredicateNode {
        self.0.child(1)
    }

    pub fn op_type(&self) -> BinOpType {
        if let PredicateType::BinOp(op_type) = self.0.typ {
            op_type
        } else {
            panic!("not a bin op")
        }
    }
}

impl ReprPredicateNode for BinOpPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if !matches!(pred_node.typ, PredicateType::BinOp(_)) {
            return None;
        }
        Some(Self(pred_node))
    }
}
