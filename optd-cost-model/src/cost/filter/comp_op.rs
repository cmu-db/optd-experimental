use std::ops::Bound;

use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{
            attr_index_pred::AttrIndexPred, bin_op_pred::BinOpType, cast_pred::CastPred,
            constant_pred::ConstantPred,
        },
        properties::attr_ref::{AttrRef, BaseTableAttrRef},
        types::GroupId,
        values::Value,
    },
    cost_model::CostModelImpl,
    stats::{DEFAULT_EQ_SEL, DEFAULT_INEQ_SEL, UNIMPLEMENTED_SEL},
    storage::CostModelStorageManager,
    CostModelResult,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// Comparison operators are the base case for recursion in get_filter_selectivity()
    pub(crate) async fn get_comp_op_selectivity(
        &self,
        group_id: GroupId,
        comp_bin_op_typ: BinOpType,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
    ) -> CostModelResult<f64> {
        assert!(comp_bin_op_typ.is_comparison());

        // I intentionally performed moves on left and right. This way, we don't accidentally use
        // them after this block
        let semantic_res = self.get_semantic_nodes(group_id, left, right).await;
        if semantic_res.is_err() {
            return Ok(Self::get_default_comparison_op_selectivity(comp_bin_op_typ));
        }
        let (attr_ref_exprs, values, non_attr_ref_exprs, is_left_attr_ref) = semantic_res.unwrap();

        // Handle the different cases of semantic nodes.
        if attr_ref_exprs.is_empty() {
            Ok(UNIMPLEMENTED_SEL)
        } else if attr_ref_exprs.len() == 1 {
            let attr_ref_expr = attr_ref_exprs
                .first()
                .expect("we just checked that attr_ref_exprs.len() == 1");
            let attr_ref_idx = attr_ref_expr.attr_index();

            if let AttrRef::BaseTableAttrRef(BaseTableAttrRef { table_id, attr_idx }) =
                self.memo.get_attribute_ref(group_id, attr_ref_idx)
            {
                if values.len() == 1 {
                    let value = values
                        .first()
                        .expect("we just checked that values.len() == 1");
                    match comp_bin_op_typ {
                        BinOpType::Eq => {
                            self.get_attribute_equality_selectivity(table_id, attr_idx, value, true)
                                .await
                        }
                        BinOpType::Neq => {
                            self.get_attribute_equality_selectivity(
                                table_id,
                                attr_ref_idx,
                                value,
                                false,
                            )
                            .await
                        }
                        BinOpType::Lt | BinOpType::Leq | BinOpType::Gt | BinOpType::Geq => {
                            let start = match (comp_bin_op_typ, is_left_attr_ref) {
                                    (BinOpType::Lt, true) | (BinOpType::Geq, false) => Bound::Unbounded,
                                    (BinOpType::Leq, true) | (BinOpType::Gt, false) => Bound::Unbounded,
                                    (BinOpType::Gt, true) | (BinOpType::Leq, false) => Bound::Excluded(value),
                                    (BinOpType::Geq, true) | (BinOpType::Lt, false) => Bound::Included(value),
                                    _ => unreachable!("all comparison BinOpTypes were enumerated. this should be unreachable"),
                                };
                            let end = match (comp_bin_op_typ, is_left_attr_ref) {
                                    (BinOpType::Lt, true) | (BinOpType::Geq, false) => Bound::Excluded(value),
                                    (BinOpType::Leq, true) | (BinOpType::Gt, false) => Bound::Included(value),
                                    (BinOpType::Gt, true) | (BinOpType::Leq, false) => Bound::Unbounded,
                                    (BinOpType::Geq, true) | (BinOpType::Lt, false) => Bound::Unbounded,
                                    _ => unreachable!("all comparison BinOpTypes were enumerated. this should be unreachable"),
                                };
                            self.get_attribute_range_selectivity(table_id, attr_ref_idx, start, end)
                                .await
                        }
                        _ => unreachable!(
                            "all comparison BinOpTypes were enumerated. this should be unreachable"
                        ),
                    }
                } else {
                    let non_attr_ref_expr = non_attr_ref_exprs.first().expect(
                        "non_attr_ref_exprs should have a value since attr_ref_exprs.len() == 1",
                    );

                    match non_attr_ref_expr.as_ref().typ {
                        PredicateType::BinOp(_) => {
                            Ok(Self::get_default_comparison_op_selectivity(comp_bin_op_typ))
                        }
                        PredicateType::Cast => Ok(UNIMPLEMENTED_SEL),
                        PredicateType::Constant(_) => {
                            unreachable!(
                                "we should have handled this in the values.len() == 1 branch"
                            )
                        }
                        _ => unimplemented!(
                            "unhandled case of comparing a attribute ref node to {}",
                            non_attr_ref_expr.as_ref().typ
                        ),
                    }
                }
            } else {
                // TODO: attribute is derived
                Ok(Self::get_default_comparison_op_selectivity(comp_bin_op_typ))
            }
        } else if attr_ref_exprs.len() == 2 {
            Ok(Self::get_default_comparison_op_selectivity(comp_bin_op_typ))
        } else {
            unreachable!("we could have at most pushed left and right into attr_ref_exprs")
        }
    }

    /// Convert the left and right child nodes of some operation to what they semantically are.
    /// This is convenient to avoid repeating the same logic just with "left" and "right" swapped.
    /// The last return value is true when the input node (left) is a AttributeRefPred.
    #[allow(clippy::type_complexity)]
    async fn get_semantic_nodes(
        &self,
        group_id: GroupId,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
    ) -> CostModelResult<(Vec<AttrIndexPred>, Vec<Value>, Vec<ArcPredicateNode>, bool)> {
        let mut attr_ref_exprs = vec![];
        let mut values = vec![];
        let mut non_attr_ref_exprs = vec![];
        let is_left_attr_ref;

        // Recursively unwrap casts as much as we can.
        let mut uncasted_left = left;
        let mut uncasted_right = right;
        loop {
            // println!("loop {}, uncasted_left={:?}, uncasted_right={:?}", Local::now(),
            // uncasted_left, uncasted_right);
            if uncasted_left.as_ref().typ == PredicateType::Cast
                && uncasted_right.as_ref().typ == PredicateType::Cast
            {
                let left_cast_expr = CastPred::from_pred_node(uncasted_left)
                    .expect("we already checked that the type is Cast");
                let right_cast_expr = CastPred::from_pred_node(uncasted_right)
                    .expect("we already checked that the type is Cast");
                assert!(left_cast_expr.cast_to() == right_cast_expr.cast_to());
                uncasted_left = left_cast_expr.child().into_pred_node();
                uncasted_right = right_cast_expr.child().into_pred_node();
            } else if uncasted_left.as_ref().typ == PredicateType::Cast
                || uncasted_right.as_ref().typ == PredicateType::Cast
            {
                let is_left_cast = uncasted_left.as_ref().typ == PredicateType::Cast;
                let (mut cast_node, mut non_cast_node) = if is_left_cast {
                    (uncasted_left, uncasted_right)
                } else {
                    (uncasted_right, uncasted_left)
                };

                let cast_expr = CastPred::from_pred_node(cast_node)
                    .expect("we already checked that the type is Cast");
                let cast_expr_child = cast_expr.child().into_pred_node();
                let cast_expr_cast_to = cast_expr.cast_to();

                let should_break = match cast_expr_child.typ {
                    PredicateType::Constant(_) => {
                        cast_node = ConstantPred::new(
                            ConstantPred::from_pred_node(cast_expr_child)
                                .expect("we already checked that the type is Constant")
                                .value()
                                .convert_to_type(cast_expr_cast_to),
                        )
                        .into_pred_node();
                        false
                    }
                    PredicateType::AttrIndex => {
                        let attr_ref_expr = AttrIndexPred::from_pred_node(cast_expr_child)
                            .expect("we already checked that the type is AttributeRef");
                        let attr_ref_idx = attr_ref_expr.attr_index();
                        cast_node = attr_ref_expr.into_pred_node();
                        // The "invert" cast is to invert the cast so that we're casting the
                        // non_cast_node to the attribute's original type.
                        let attribute_info = self.memo.get_attribute_info(group_id, attr_ref_idx);
                        let invert_cast_data_type = &attribute_info.typ.into_data_type();

                        match non_cast_node.typ {
                            PredicateType::AttrIndex => {
                                // In general, there's no way to remove the Cast here. We can't move
                                // the Cast to the other AttributeRef
                                // because that would lead to an infinite loop. Thus, we just leave
                                // the cast where it is and break.
                                true
                            }
                            _ => {
                                non_cast_node =
                                    CastPred::new(non_cast_node, invert_cast_data_type.clone())
                                        .into_pred_node();
                                false
                            }
                        }
                    }
                    _ => todo!(),
                };

                (uncasted_left, uncasted_right) = if is_left_cast {
                    (cast_node, non_cast_node)
                } else {
                    (non_cast_node, cast_node)
                };

                if should_break {
                    break;
                }
            } else {
                break;
            }
        }

        // Sort nodes into attr_ref_exprs, values, and non_attr_ref_exprs
        match uncasted_left.as_ref().typ {
            PredicateType::AttrIndex => {
                is_left_attr_ref = true;
                attr_ref_exprs.push(
                    AttrIndexPred::from_pred_node(uncasted_left)
                        .expect("we already checked that the type is AttributeRef"),
                );
            }
            PredicateType::Constant(_) => {
                is_left_attr_ref = false;
                values.push(
                    ConstantPred::from_pred_node(uncasted_left)
                        .expect("we already checked that the type is Constant")
                        .value(),
                )
            }
            _ => {
                is_left_attr_ref = false;
                non_attr_ref_exprs.push(uncasted_left);
            }
        }
        match uncasted_right.as_ref().typ {
            PredicateType::AttrIndex => {
                attr_ref_exprs.push(
                    AttrIndexPred::from_pred_node(uncasted_right)
                        .expect("we already checked that the type is AttributeRef"),
                );
            }
            PredicateType::Constant(_) => values.push(
                ConstantPred::from_pred_node(uncasted_right)
                    .expect("we already checked that the type is Constant")
                    .value(),
            ),
            _ => {
                non_attr_ref_exprs.push(uncasted_right);
            }
        }

        assert!(attr_ref_exprs.len() + values.len() + non_attr_ref_exprs.len() == 2);
        Ok((attr_ref_exprs, values, non_attr_ref_exprs, is_left_attr_ref))
    }

    /// The default selectivity of a comparison expression
    /// Used when one side of the comparison is a attribute while the other side is something too
    ///   complex/impossible to evaluate (subquery, UDF, another attribute, we have no stats, etc.)
    fn get_default_comparison_op_selectivity(comp_bin_op_typ: BinOpType) -> f64 {
        assert!(comp_bin_op_typ.is_comparison());
        match comp_bin_op_typ {
            BinOpType::Eq => DEFAULT_EQ_SEL,
            BinOpType::Neq => 1.0 - DEFAULT_EQ_SEL,
            BinOpType::Lt | BinOpType::Leq | BinOpType::Gt | BinOpType::Geq => DEFAULT_INEQ_SEL,
            _ => unreachable!(
                "all comparison BinOpTypes were enumerated. this should be unreachable"
            ),
        }
    }
}
