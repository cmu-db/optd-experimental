use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    values::Value,
};

use super::list_pred::ListPred;

#[derive(Clone, Debug)]
pub struct InListPred(pub ArcPredicateNode);

impl InListPred {
    pub fn new(child: ArcPredicateNode, list: ListPred, negated: bool) -> Self {
        InListPred(
            PredicateNode {
                typ: PredicateType::InList,
                children: vec![child, list.into_pred_node()],
                data: Some(Value::Bool(negated)),
            }
            .into(),
        )
    }

    pub fn child(&self) -> ArcPredicateNode {
        self.0.child(0)
    }

    pub fn list(&self) -> ListPred {
        ListPred::from_pred_node(self.0.child(1)).unwrap()
    }

    /// `true` for `NOT IN`.
    pub fn negated(&self) -> bool {
        self.0.data.as_ref().unwrap().as_bool()
    }
}

impl ReprPredicateNode for InListPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if !matches!(pred_node.typ, PredicateType::InList) {
            return None;
        }
        Some(Self(pred_node))
    }
}
