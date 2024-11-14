#![allow(unused_variables)]
use optd_persistent::CostModelStorageLayer;

use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{
            attr_ref_pred::AttributeRefPred,
            bin_op_pred::BinOpType,
            cast_pred::CastPred,
            constant_pred::{ConstantPred, ConstantType},
            un_op_pred::UnOpType,
        },
        values::Value,
    },
    cost_model::CostModelImpl,
    CostModelResult, EstimatedStatistic,
};

// A placeholder for unimplemented!() for codepaths which are accessed by plannertest
const UNIMPLEMENTED_SEL: f64 = 0.01;
// Default statistics. All are from selfuncs.h in Postgres unless specified otherwise
// Default selectivity estimate for equalities such as "A = b"
const DEFAULT_EQ_SEL: f64 = 0.005;
// Default selectivity estimate for inequalities such as "A < b"
const DEFAULT_INEQ_SEL: f64 = 0.3333333333333333;

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    pub fn get_filter_row_cnt(
        &self,
        child_row_cnt: EstimatedStatistic,
        table_id: i32,
        cond: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let selectivity = { self.get_filter_selectivity(cond, table_id)? };
        Ok(
            EstimatedStatistic((child_row_cnt.0 as f64 * selectivity) as u64)
                .max(EstimatedStatistic(1)),
        )
    }

    pub fn get_filter_selectivity(
        &self,
        expr_tree: ArcPredicateNode,
        table_id: i32,
    ) -> CostModelResult<f64> {
        match &expr_tree.typ {
            PredicateType::Constant(_) => Ok(Self::get_constant_selectivity(expr_tree)),
            PredicateType::AttributeRef => unimplemented!("check bool type or else panic"),
            PredicateType::UnOp(un_op_typ) => {
                assert!(expr_tree.children.len() == 1);
                let child = expr_tree.child(0);
                match un_op_typ {
                    // not doesn't care about nulls so there's no complex logic. it just reverses
                    // the selectivity for instance, != _will not_ include nulls
                    // but "NOT ==" _will_ include nulls
                    UnOpType::Not => Ok(1.0 - self.get_filter_selectivity(child, table_id)?),
                    UnOpType::Neg => panic!(
                        "the selectivity of operations that return numerical values is undefined"
                    ),
                }
            }
            PredicateType::BinOp(bin_op_typ) => {
                assert!(expr_tree.children.len() == 2);
                let left_child = expr_tree.child(0);
                let right_child = expr_tree.child(1);

                if bin_op_typ.is_comparison() {
                    self.get_comp_op_selectivity(*bin_op_typ, left_child, right_child, table_id)
                } else if bin_op_typ.is_numerical() {
                    panic!(
                        "the selectivity of operations that return numerical values is undefined"
                    )
                } else {
                    unreachable!("all BinOpTypes should be true for at least one is_*() function")
                }
            }
            _ => unimplemented!("check bool type or else panic"),
        }
    }

    fn get_constant_selectivity(const_node: ArcPredicateNode) -> f64 {
        if let PredicateType::Constant(const_typ) = const_node.typ {
            if matches!(const_typ, ConstantType::Bool) {
                let value = const_node
                    .as_ref()
                    .data
                    .as_ref()
                    .expect("constants should have data");
                if let Value::Bool(bool_value) = value {
                    if *bool_value {
                        1.0
                    } else {
                        0.0
                    }
                } else {
                    unreachable!(
                        "if the typ is ConstantType::Bool, the value should be a Value::Bool"
                    )
                }
            } else {
                panic!("selectivity is not defined on constants which are not bools")
            }
        } else {
            panic!("get_constant_selectivity must be called on a constant")
        }
    }

    /// Comparison operators are the base case for recursion in get_filter_selectivity()
    fn get_comp_op_selectivity(
        &self,
        comp_bin_op_typ: BinOpType,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
        table_id: i32,
    ) -> CostModelResult<f64> {
        assert!(comp_bin_op_typ.is_comparison());

        // I intentionally performed moves on left and right. This way, we don't accidentally use
        // them after this block
        let (col_ref_exprs, values, non_col_ref_exprs, is_left_col_ref) =
            self.get_semantic_nodes(left, right, table_id)?;

        // Handle the different cases of semantic nodes.
        if col_ref_exprs.is_empty() {
            Ok(UNIMPLEMENTED_SEL)
        } else if col_ref_exprs.len() == 1 {
            let col_ref_expr = col_ref_exprs
                .first()
                .expect("we just checked that col_ref_exprs.len() == 1");
            let col_ref_idx = col_ref_expr.index();

            todo!()
        } else if col_ref_exprs.len() == 2 {
            Ok(Self::get_default_comparison_op_selectivity(comp_bin_op_typ))
        } else {
            unreachable!("we could have at most pushed left and right into col_ref_exprs")
        }
    }

    /// Convert the left and right child nodes of some operation to what they semantically are.
    /// This is convenient to avoid repeating the same logic just with "left" and "right" swapped.
    /// The last return value is true when the input node (left) is a ColumnRefPred.
    #[allow(clippy::type_complexity)]
    fn get_semantic_nodes(
        &self,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
        table_id: i32,
    ) -> CostModelResult<(
        Vec<AttributeRefPred>,
        Vec<Value>,
        Vec<ArcPredicateNode>,
        bool,
    )> {
        let mut col_ref_exprs = vec![];
        let mut values = vec![];
        let mut non_col_ref_exprs = vec![];
        let is_left_col_ref;

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
                    PredicateType::AttributeRef => {
                        let col_ref_expr = AttributeRefPred::from_pred_node(cast_expr_child)
                            .expect("we already checked that the type is ColumnRef");
                        let col_ref_idx = col_ref_expr.index();
                        cast_node = col_ref_expr.into_pred_node();
                        // The "invert" cast is to invert the cast so that we're casting the
                        // non_cast_node to the column's original type.
                        // TODO(migration): double check
                        let invert_cast_data_type = &(self
                            .storage_manager
                            .get_attribute_info(table_id, col_ref_idx as i32)?
                            .typ
                            .into_data_type());

                        match non_cast_node.typ {
                            PredicateType::AttributeRef => {
                                // In general, there's no way to remove the Cast here. We can't move
                                // the Cast to the other ColumnRef
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

        // Sort nodes into col_ref_exprs, values, and non_col_ref_exprs
        match uncasted_left.as_ref().typ {
            PredicateType::AttributeRef => {
                is_left_col_ref = true;
                col_ref_exprs.push(
                    AttributeRefPred::from_pred_node(uncasted_left)
                        .expect("we already checked that the type is ColumnRef"),
                );
            }
            PredicateType::Constant(_) => {
                is_left_col_ref = false;
                values.push(
                    ConstantPred::from_pred_node(uncasted_left)
                        .expect("we already checked that the type is Constant")
                        .value(),
                )
            }
            _ => {
                is_left_col_ref = false;
                non_col_ref_exprs.push(uncasted_left);
            }
        }
        match uncasted_right.as_ref().typ {
            PredicateType::AttributeRef => {
                col_ref_exprs.push(
                    AttributeRefPred::from_pred_node(uncasted_right)
                        .expect("we already checked that the type is ColumnRef"),
                );
            }
            PredicateType::Constant(_) => values.push(
                ConstantPred::from_pred_node(uncasted_right)
                    .expect("we already checked that the type is Constant")
                    .value(),
            ),
            _ => {
                non_col_ref_exprs.push(uncasted_right);
            }
        }

        assert!(col_ref_exprs.len() + values.len() + non_col_ref_exprs.len() == 2);
        Ok((col_ref_exprs, values, non_col_ref_exprs, is_left_col_ref))
    }

    /// The default selectivity of a comparison expression
    /// Used when one side of the comparison is a column while the other side is something too
    ///   complex/impossible to evaluate (subquery, UDF, another column, we have no stats, etc.)
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
