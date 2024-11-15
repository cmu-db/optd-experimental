use crate::common::nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode};

use super::id_pred::IdPred;

/// [`AttributeRefPred`] represents a reference to a column in a relation.
///
/// An [`AttributeRefPred`] has two children:
/// 1. The table id, represented by an [`IdPred`].
/// 2. The index of the column, represented by an [`IdPred`].
///
/// Currently, [`AttributeRefPred`] only holds base table attributes, i.e. attributes
/// that already exist in the table. More complex structures may be introduced in the
/// future to represent derived attributes (e.g. t.v1 + t.v2).
#[derive(Clone, Debug)]
pub struct AttributeRefPred(pub ArcPredicateNode);

impl AttributeRefPred {
    pub fn new(table_id: usize, attribute_idx: usize) -> AttributeRefPred {
        AttributeRefPred(
            PredicateNode {
                typ: PredicateType::AttributeRef,
                children: vec![
                    IdPred::new(table_id).into_pred_node(),
                    IdPred::new(attribute_idx).into_pred_node(),
                ],
                data: None,
            }
            .into(),
        )
    }

    /// Gets the table id.
    pub fn table_id(&self) -> usize {
        self.0.child(0).data.as_ref().unwrap().as_u64() as usize
    }

    /// Gets the attribute index.
    pub fn attr_index(&self) -> usize {
        self.0.child(1).data.as_ref().unwrap().as_u64() as usize
    }
}

impl ReprPredicateNode for AttributeRefPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        if pred_node.typ != PredicateType::AttributeRef {
            return None;
        }
        Some(Self(pred_node))
    }
}
