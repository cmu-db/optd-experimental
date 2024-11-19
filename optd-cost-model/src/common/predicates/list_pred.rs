use crate::common::nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode};

#[derive(Clone, Debug)]
pub struct ListPred(pub ArcPredicateNode);

impl ListPred {
    pub fn new(preds: Vec<ArcPredicateNode>) -> Self {
        ListPred(
            PredicateNode {
                typ: PredicateType::List,
                children: preds,
                data: None,
            }
            .into(),
        )
    }

    /// Gets number of expressions in the list
    pub fn len(&self) -> usize {
        self.0.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.children.is_empty()
    }

    pub fn child(&self, idx: usize) -> ArcPredicateNode {
        self.0.child(idx)
    }

    pub fn to_vec(&self) -> Vec<ArcPredicateNode> {
        self.0.children.clone()
    }
}

impl ReprPredicateNode for ListPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if pred_node.typ != PredicateType::List {
            return None;
        }
        Some(Self(pred_node))
    }
}
