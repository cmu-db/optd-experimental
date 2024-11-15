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
    pub fn new(id: usize) -> IdPred {
        // This conversion is always safe since usize is at most u64.
        let u64_id = id as u64;
        IdPred(
            PredicateNode {
                typ: PredicateType::Id,
                children: vec![],
                data: Some(Value::UInt64(u64_id)),
            }
            .into(),
        )
    }

    /// Gets the id stored in the predicate.
    pub fn id(&self) -> usize {
        self.0.data.clone().unwrap().as_u64() as usize
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
