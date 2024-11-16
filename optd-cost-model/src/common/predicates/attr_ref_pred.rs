use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    types::TableId,
};

use super::id_pred::IdPred;

/// [`AttributeRefPred`] represents a reference to a column in a relation.
///
/// An [`AttributeRefPred`] has two children:
/// 1. The table id, represented by an [`IdPred`].
/// 2. The index of the attribute, represented by an [`IdPred`].
///
/// Although it may be strange at first glance (table id and attribute base index
/// aren't children of the attribute reference), but considering the attribute reference
/// can be represented as table_id.attr_base_index, and it enables the cost model to
/// obtain the information in a simple way without refactoring `data` field.
///
/// **TODO**: Now we assume any IdPred is as same as the ones in the ORM layer.
///
/// Currently, [`AttributeRefPred`] only holds base table attributes, i.e. attributes
/// that already exist in the table. More complex structures may be introduced in the
/// future to represent derived attributes (e.g. t.v1 + t.v2).
///
/// TODO: Support derived column in `AttributeRefPred`.
/// Proposal: Data field can store the column type (base or derived).
#[derive(Clone, Debug)]
pub struct AttributeRefPred(pub ArcPredicateNode);

impl AttributeRefPred {
    pub fn new(table_id: TableId, attribute_idx: usize) -> AttributeRefPred {
        AttributeRefPred(
            PredicateNode {
                typ: PredicateType::AttributeRef,
                children: vec![
                    IdPred::new(table_id.0).into_pred_node(),
                    IdPred::new(attribute_idx).into_pred_node(),
                ],
                data: None,
            }
            .into(),
        )
    }

    /// Gets the table id.
    pub fn table_id(&self) -> TableId {
        TableId(self.0.child(0).data.as_ref().unwrap().as_u64() as usize)
    }

    /// Gets the attribute index.
    /// Note: The attribute index is the **base** index, which is table specific.
    pub fn attr_index(&self) -> usize {
        self.0.child(1).data.as_ref().unwrap().as_u64() as usize
    }

    /// Checks whether the attribute is a derived attribute. Currently, this will always return
    /// false, since derived attribute is not yet supported.
    pub fn is_derived(&self) -> bool {
        false
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
