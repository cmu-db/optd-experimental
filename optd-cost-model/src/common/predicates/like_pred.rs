use std::sync::Arc;

use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    values::Value,
};

#[derive(Clone, Debug)]
pub struct LikePred(pub ArcPredicateNode);

impl LikePred {
    pub fn new(
        negated: bool,
        case_insensitive: bool,
        child: ArcPredicateNode,
        pattern: ArcPredicateNode,
    ) -> Self {
        // TODO: support multiple values in data.
        let negated = if negated { 1 } else { 0 };
        let case_insensitive = if case_insensitive { 1 } else { 0 };
        LikePred(
            PredicateNode {
                typ: PredicateType::Like,
                children: vec![child.into_pred_node(), pattern.into_pred_node()],
                data: Some(Value::Serialized(Arc::new([negated, case_insensitive]))),
            }
            .into(),
        )
    }

    pub fn child(&self) -> ArcPredicateNode {
        self.0.child(0)
    }

    pub fn pattern(&self) -> ArcPredicateNode {
        self.0.child(1)
    }

    /// `true` for `NOT LIKE`.
    pub fn negated(&self) -> bool {
        match self.0.data.as_ref().unwrap() {
            Value::Serialized(data) => data[0] != 0,
            _ => panic!("not a serialized value"),
        }
    }

    pub fn case_insensitive(&self) -> bool {
        match self.0.data.as_ref().unwrap() {
            Value::Serialized(data) => data[1] != 0,
            _ => panic!("not a serialized value"),
        }
    }
}

impl ReprPredicateNode for LikePred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if !matches!(pred_node.typ, PredicateType::Like) {
            return None;
        }
        Some(Self(pred_node))
    }
}
