use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    values::Value,
};

/// [`IdPred`] holds an id or an index, e.g. table id.
///
/// The data is of uint64 type, because an id or an index can always be
/// represented by uint64.
#[derive(Clone, Debug)]
pub struct IdPred(pub ArcPredicateNode);

impl IdPred {
    pub fn new(id: u64) -> IdPred {
        IdPred(
            PredicateNode {
                typ: PredicateType::Id,
                children: vec![],
                data: Some(Value::UInt64(id)),
            }
            .into(),
        )
    }

    /// Gets the id stored in the predicate.
    pub fn id(&self) -> u64 {
        self.0.data.clone().unwrap().as_u64()
    }
}

impl ReprPredicateNode for IdPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if let PredicateType::Id = pred_node.typ {
            Some(Self(pred_node))
        } else {
            None
        }
    }
}
