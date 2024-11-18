use crate::common::{
    nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
    predicates::{attr_ref_pred::AttrRefPred, bin_op_pred::BinOpType},
    properties::attr_ref::{
        AttrRef, AttrRefs, BaseTableAttrRef, GroupAttrRefs, SemanticCorrelation,
    },
};

pub mod hash_join;
pub mod join;
pub mod nested_loop_join;

pub(crate) fn get_input_correlation(
    left_prop: GroupAttrRefs,
    right_prop: GroupAttrRefs,
) -> Option<SemanticCorrelation> {
    SemanticCorrelation::merge(
        left_prop.output_correlation().cloned(),
        right_prop.output_correlation().cloned(),
    )
}

/// Check if an expr_tree is a join condition, returning the join on attr ref pair if it is.
/// The reason the check and the info are in the same function is because their code is almost
/// identical. It only picks out equality conditions between two attribute refs on different
/// tables
pub(crate) fn get_on_attr_ref_pair(
    expr_tree: ArcPredicateNode,
    attr_refs: &AttrRefs,
) -> Option<(AttrRefPred, AttrRefPred)> {
    // 1. Check that it's equality
    if expr_tree.typ == PredicateType::BinOp(BinOpType::Eq) {
        let left_child = expr_tree.child(0);
        let right_child = expr_tree.child(1);
        // 2. Check that both sides are attribute refs
        if left_child.typ == PredicateType::AttrRef && right_child.typ == PredicateType::AttrRef {
            // 3. Check that both sides don't belong to the same table (if we don't know, that
            //    means they don't belong)
            let left_attr_ref_expr = AttrRefPred::from_pred_node(left_child)
                .expect("we already checked that the type is AttrRef");
            let right_attr_ref_expr = AttrRefPred::from_pred_node(right_child)
                .expect("we already checked that the type is AttrRef");
            let left_attr_ref = &attr_refs[left_attr_ref_expr.attr_index() as usize];
            let right_attr_ref = &attr_refs[right_attr_ref_expr.attr_index() as usize];
            let is_same_table = if let (
                AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                    table_id: left_table_id,
                    ..
                }),
                AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                    table_id: right_table_id,
                    ..
                }),
            ) = (left_attr_ref, right_attr_ref)
            {
                left_table_id == right_table_id
            } else {
                false
            };
            if !is_same_table {
                Some((left_attr_ref_expr, right_attr_ref_expr))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
