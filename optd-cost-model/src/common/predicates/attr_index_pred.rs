use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    values::Value,
};

/// [`AttributeIndexPred`] represents the position of an attribute in a schema or
/// [`GroupAttrRefs`].
///
/// The `data` field holds the index of the attribute in the schema or [`GroupAttrRefs`].
#[derive(Clone, Debug)]
pub struct AttrIndexPred(pub ArcPredicateNode);

impl AttrIndexPred {
    pub fn new(attr_idx: u64) -> AttrIndexPred {
        AttrIndexPred(
            PredicateNode {
                typ: PredicateType::AttrIndex,
                children: vec![],
                data: Some(Value::UInt64(attr_idx)),
            }
            .into(),
        )
    }

    /// Gets the attribute index.
    pub fn attr_index(&self) -> u64 {
        self.0.data.as_ref().unwrap().as_u64()
    }
}

impl ReprPredicateNode for AttrIndexPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if pred_node.typ != PredicateType::AttrIndex {
            return None;
        }
        Some(Self(pred_node))
    }
}
