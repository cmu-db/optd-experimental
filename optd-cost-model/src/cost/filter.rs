#![allow(unused_variables)]
use std::ops::Bound;

use datafusion::arrow::array::StringArray;
use datafusion::arrow::compute::like;
use optd_persistent::{cost_model::interface::Cost, CostModelStorageLayer};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{
            attr_ref_pred::AttributeRefPred,
            bin_op_pred::BinOpType,
            cast_pred::CastPred,
            constant_pred::{ConstantPred, ConstantType},
            in_list_pred::InListPred,
            like_pred::LikePred,
            log_op_pred::LogOpType,
            un_op_pred::UnOpType,
        },
        types::TableId,
        values::Value,
    },
    cost_model::CostModelImpl,
    stats::{
        AttributeCombValue, AttributeCombValueStats, DEFAULT_EQ_SEL, DEFAULT_INEQ_SEL,
        FIXED_CHAR_SEL_FACTOR, FULL_WILDCARD_SEL_FACTOR, UNIMPLEMENTED_SEL,
    },
    CostModelResult, EstimatedStatistic,
};

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    // TODO: is it a good design to pass table_id here? I think it needs to be refactored.
    // Consider to remove table_id.
    pub fn get_filter_row_cnt(
        &self,
        child_row_cnt: EstimatedStatistic,
        table_id: TableId,
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
        table_id: TableId,
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
            PredicateType::LogOp(log_op_typ) => {
                self.get_log_op_selectivity(*log_op_typ, &expr_tree.children, table_id)
            }
            PredicateType::Func(_) => unimplemented!("check bool type or else panic"),
            PredicateType::SortOrder(_) => {
                panic!("the selectivity of sort order expressions is undefined")
            }
            PredicateType::Between => Ok(UNIMPLEMENTED_SEL),
            PredicateType::Cast => unimplemented!("check bool type or else panic"),
            PredicateType::Like => {
                let like_expr = LikePred::from_pred_node(expr_tree).unwrap();
                self.get_like_selectivity(&like_expr, table_id)
            }
            PredicateType::DataType(_) => {
                panic!("the selectivity of a data type is not defined")
            }
            PredicateType::InList => {
                let in_list_expr = InListPred::from_pred_node(expr_tree).unwrap();
                self.get_in_list_selectivity(&in_list_expr, table_id)
            }
            _ => unreachable!(
                "all expression DfPredType were enumerated. this should be unreachable"
            ),
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

    fn get_log_op_selectivity(
        &self,
        log_op_typ: LogOpType,
        children: &[ArcPredicateNode],
        table_id: TableId,
    ) -> CostModelResult<f64> {
        match log_op_typ {
            LogOpType::And => children.iter().try_fold(1.0, |acc, child| {
                let selectivity = self.get_filter_selectivity(child.clone(), table_id)?;
                Ok(acc * selectivity)
            }),
            LogOpType::Or => {
                let product = children.iter().try_fold(1.0, |acc, child| {
                    let selectivity = self.get_filter_selectivity(child.clone(), table_id)?;
                    Ok(acc * (1.0 - selectivity))
                })?;
                Ok(1.0 - product)
            }
        }
    }

    /// Comparison operators are the base case for recursion in get_filter_selectivity()
    fn get_comp_op_selectivity(
        &self,
        comp_bin_op_typ: BinOpType,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
        table_id: TableId,
    ) -> CostModelResult<f64> {
        assert!(comp_bin_op_typ.is_comparison());

        // I intentionally performed moves on left and right. This way, we don't accidentally use
        // them after this block
        let (attr_ref_exprs, values, non_attr_ref_exprs, is_left_attr_ref) =
            self.get_semantic_nodes(left, right, table_id)?;

        // Handle the different cases of semantic nodes.
        if attr_ref_exprs.is_empty() {
            Ok(UNIMPLEMENTED_SEL)
        } else if attr_ref_exprs.len() == 1 {
            let attr_ref_expr = attr_ref_exprs
                .first()
                .expect("we just checked that attr_ref_exprs.len() == 1");
            let attr_ref_idx = attr_ref_expr.attr_index();

            // TODO: Consider attribute is a derived attribute
            if values.len() == 1 {
                let value = values
                    .first()
                    .expect("we just checked that values.len() == 1");
                match comp_bin_op_typ {
                    BinOpType::Eq => {
                        self.get_attribute_equality_selectivity(table_id, attr_ref_idx, value, true)
                    }
                    BinOpType::Neq => self.get_attribute_equality_selectivity(
                        table_id,
                        attr_ref_idx,
                        value,
                        false,
                    ),
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
                        unreachable!("we should have handled this in the values.len() == 1 branch")
                    }
                    _ => unimplemented!(
                        "unhandled case of comparing a attribute ref node to {}",
                        non_attr_ref_expr.as_ref().typ
                    ),
                }
            }
        } else if attr_ref_exprs.len() == 2 {
            Ok(Self::get_default_comparison_op_selectivity(comp_bin_op_typ))
        } else {
            unreachable!("we could have at most pushed left and right into attr_ref_exprs")
        }
    }

    /// Get the selectivity of an expression of the form "attribute equals value" (or "value equals
    /// attribute") Will handle the case of statistics missing
    /// Equality predicates are handled entirely differently from range predicates so this is its
    /// own function
    /// Also, get_attribute_equality_selectivity is a subroutine when computing range
    /// selectivity, which is another     reason for separating these into two functions
    /// is_eq means whether it's == or !=
    fn get_attribute_equality_selectivity(
        &self,
        table_id: TableId,
        attr_base_index: usize,
        value: &Value,
        is_eq: bool,
    ) -> CostModelResult<f64> {
        // TODO: The attribute could be a derived attribute
        let ret_sel = {
            // TODO: Handle the case where `attribute_stats` is None.
            let attribute_stats = self
                .get_attribute_comb_stats(table_id, &[attr_base_index])?
                .unwrap();
            let eq_freq = if let Some(freq) = attribute_stats.mcvs.freq(&vec![Some(value.clone())])
            {
                freq
            } else {
                let non_mcv_freq = 1.0 - attribute_stats.mcvs.total_freq();
                // always safe because usize is at least as large as i32
                let ndistinct_as_usize = attribute_stats.ndistinct as usize;
                let non_mcv_cnt = ndistinct_as_usize - attribute_stats.mcvs.cnt();
                if non_mcv_cnt == 0 {
                    return Ok(0.0);
                }
                // note that nulls are not included in ndistinct so we don't need to do non_mcv_cnt
                // - 1 if null_frac > 0
                (non_mcv_freq - attribute_stats.null_frac) / (non_mcv_cnt as f64)
            };
            if is_eq {
                eq_freq
            } else {
                1.0 - eq_freq - attribute_stats.null_frac
            }
        };

        assert!(
            (0.0..=1.0).contains(&ret_sel),
            "ret_sel ({}) should be in [0, 1]",
            ret_sel
        );
        Ok(ret_sel)
    }

    /// Compute the frequency of values in a attribute less than or equal to the given value.
    fn get_attribute_leq_value_freq(
        per_attribute_stats: &AttributeCombValueStats,
        value: &Value,
    ) -> f64 {
        // because distr does not include the values in MCVs, we need to compute the CDFs there as
        // well because nulls return false in any comparison, they are never included when
        // computing range selectivity
        let distr_leq_freq = per_attribute_stats.distr.as_ref().unwrap().cdf(value);
        let value = value.clone();
        let pred = Box::new(move |val: &AttributeCombValue| *val[0].as_ref().unwrap() <= value);
        let mcvs_leq_freq = per_attribute_stats.mcvs.freq_over_pred(pred);
        let ret_freq = distr_leq_freq + mcvs_leq_freq;
        assert!(
            (0.0..=1.0).contains(&ret_freq),
            "ret_freq ({}) should be in [0, 1]",
            ret_freq
        );
        ret_freq
    }

    /// Compute the frequency of values in a attribute less than the given value.
    fn get_attribute_lt_value_freq(
        &self,
        attribute_stats: &AttributeCombValueStats,
        table_id: TableId,
        attr_base_index: usize,
        value: &Value,
    ) -> CostModelResult<f64> {
        // depending on whether value is in mcvs or not, we use different logic to turn total_lt_cdf
        // into total_leq_cdf this logic just so happens to be the exact same logic as
        // get_attribute_equality_selectivity implements
        let ret_freq = Self::get_attribute_leq_value_freq(attribute_stats, value)
            - self.get_attribute_equality_selectivity(table_id, attr_base_index, value, true)?;
        assert!(
            (0.0..=1.0).contains(&ret_freq),
            "ret_freq ({}) should be in [0, 1]",
            ret_freq
        );
        Ok(ret_freq)
    }

    /// Get the selectivity of an expression of the form "attribute </<=/>=/> value" (or "value
    /// </<=/>=/> attribute"). Computes selectivity based off of statistics.
    /// Range predicates are handled entirely differently from equality predicates so this is its
    /// own function. If it is unable to find the statistics, it returns DEFAULT_INEQ_SEL.
    /// The selectivity is computed as quantile of the right bound minus quantile of the left bound.
    fn get_attribute_range_selectivity(
        &self,
        table_id: TableId,
        attr_base_index: usize,
        start: Bound<&Value>,
        end: Bound<&Value>,
    ) -> CostModelResult<f64> {
        // TODO: Consider attribute is a derived attribute
        // TODO: Handle the case where `attribute_stats` is None.
        let attribute_stats = self
            .get_attribute_comb_stats(table_id, &[attr_base_index])?
            .unwrap();
        let left_quantile = match start {
            Bound::Unbounded => 0.0,
            Bound::Included(value) => self.get_attribute_lt_value_freq(
                &attribute_stats,
                table_id,
                attr_base_index,
                value,
            )?,
            Bound::Excluded(value) => Self::get_attribute_leq_value_freq(&attribute_stats, value),
        };
        let right_quantile = match end {
            Bound::Unbounded => 1.0,
            Bound::Included(value) => Self::get_attribute_leq_value_freq(&attribute_stats, value),
            Bound::Excluded(value) => self.get_attribute_lt_value_freq(
                &attribute_stats,
                table_id,
                attr_base_index,
                value,
            )?,
        };
        assert!(
            left_quantile <= right_quantile,
            "left_quantile ({}) should be <= right_quantile ({})",
            left_quantile,
            right_quantile
        );
        Ok(right_quantile - left_quantile)
    }

    /// Compute the selectivity of a (NOT) LIKE expression.
    ///
    /// The logic is somewhat similar to Postgres but different. Postgres first estimates the
    /// histogram part of the population and then add up data for any MCV values. If the
    /// histogram is large enough, it just uses the number of matches in the histogram,
    /// otherwise it estimates the fixed prefix and remainder of pattern separately and
    /// combine them.
    ///
    /// Our approach is simpler and less selective. Firstly, we don't use histogram. The selectivity
    /// is composed of MCV frequency and non-MCV selectivity. MCV frequency is computed by
    /// adding up frequencies of MCVs that match the pattern. Non-MCV  selectivity is computed
    /// in the same way that Postgres computes selectivity for the wildcard part of the pattern.
    fn get_like_selectivity(
        &self,
        like_expr: &LikePred,
        table_id: TableId,
    ) -> CostModelResult<f64> {
        let child = like_expr.child();

        // Check child is a attribute ref.
        if !matches!(child.typ, PredicateType::AttributeRef) {
            return Ok(UNIMPLEMENTED_SEL);
        }

        // Check pattern is a constant.
        let pattern = like_expr.pattern();
        if !matches!(pattern.typ, PredicateType::Constant(_)) {
            return Ok(UNIMPLEMENTED_SEL);
        }

        let attr_ref_idx = AttributeRefPred::from_pred_node(child)
            .unwrap()
            .attr_index();

        // TODO: Consider attribute is a derived attribute
        let pattern = ConstantPred::from_pred_node(pattern)
            .expect("we already checked pattern is a constant")
            .value()
            .as_str();

        // Compute the selectivity exculuding MCVs.
        // See Postgres `like_selectivity`.
        let non_mcv_sel = pattern
            .chars()
            .fold(1.0, |acc, c| {
                if c == '%' {
                    acc * FULL_WILDCARD_SEL_FACTOR
                } else {
                    acc * FIXED_CHAR_SEL_FACTOR
                }
            })
            .min(1.0);

        // Compute the selectivity in MCVs.
        // TODO: Handle the case where `attribute_stats` is None.
        let attribute_stats = self
            .get_attribute_comb_stats(table_id, &[attr_ref_idx])?
            .unwrap();
        let (mcv_freq, null_frac) = {
            let pred = Box::new(move |val: &AttributeCombValue| {
                let string = StringArray::from(vec![val[0].as_ref().unwrap().as_str().as_ref()]);
                let pattern = StringArray::from(vec![pattern.as_ref()]);
                like(&string, &pattern).unwrap().value(0)
            });
            (
                attribute_stats.mcvs.freq_over_pred(pred),
                attribute_stats.null_frac,
            )
        };

        let result = non_mcv_sel + mcv_freq;

        Ok(if like_expr.negated() {
            1.0 - result - null_frac
        } else {
            result
        }
        // Postgres clamps the result after histogram and before MCV. See Postgres
        // `patternsel_common`.
        .clamp(0.0001, 0.9999))
    }

    /// Only support attrA in (val1, val2, val3) where attrA is a attribute ref and
    /// val1, val2, val3 are constants.
    pub fn get_in_list_selectivity(
        &self,
        expr: &InListPred,
        table_id: TableId,
    ) -> CostModelResult<f64> {
        let child = expr.child();

        // Check child is a attribute ref.
        if !matches!(child.typ, PredicateType::AttributeRef) {
            return Ok(UNIMPLEMENTED_SEL);
        }

        // Check all expressions in the list are constants.
        let list_exprs = expr.list().to_vec();
        if list_exprs
            .iter()
            .any(|expr| !matches!(expr.typ, PredicateType::Constant(_)))
        {
            return Ok(UNIMPLEMENTED_SEL);
        }

        // Convert child and const expressions to concrete types.
        let attr_ref_idx = AttributeRefPred::from_pred_node(child)
            .unwrap()
            .attr_index();
        let list_exprs = list_exprs
            .into_iter()
            .map(|expr| {
                ConstantPred::from_pred_node(expr)
                    .expect("we already checked all list elements are constants")
            })
            .collect::<Vec<_>>();
        let negated = expr.negated();

        // TODO: Consider attribute is a derived attribute
        let in_sel = list_exprs
            .iter()
            .try_fold(0.0, |acc, expr| {
                let selectivity = self.get_attribute_equality_selectivity(
                    table_id,
                    attr_ref_idx,
                    &expr.value(),
                    /* is_equality */ true,
                )?;
                Ok(acc + selectivity)
            })?
            .min(1.0);
        if negated {
            Ok(1.0 - in_sel)
        } else {
            Ok(in_sel)
        }
    }

    /// Convert the left and right child nodes of some operation to what they semantically are.
    /// This is convenient to avoid repeating the same logic just with "left" and "right" swapped.
    /// The last return value is true when the input node (left) is a AttributeRefPred.
    #[allow(clippy::type_complexity)]
    fn get_semantic_nodes(
        &self,
        left: ArcPredicateNode,
        right: ArcPredicateNode,
        table_id: TableId,
    ) -> CostModelResult<(
        Vec<AttributeRefPred>,
        Vec<Value>,
        Vec<ArcPredicateNode>,
        bool,
    )> {
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
                    PredicateType::AttributeRef => {
                        let attr_ref_expr = AttributeRefPred::from_pred_node(cast_expr_child)
                            .expect("we already checked that the type is AttributeRef");
                        let attr_ref_idx = attr_ref_expr.attr_index();
                        cast_node = attr_ref_expr.into_pred_node();
                        // The "invert" cast is to invert the cast so that we're casting the
                        // non_cast_node to the attribute's original type.
                        // TODO(migration): double check
                        let invert_cast_data_type = &(self
                            .storage_manager
                            .get_attribute_info(table_id, attr_ref_idx as i32)?
                            .typ
                            .into_data_type());

                        match non_cast_node.typ {
                            PredicateType::AttributeRef => {
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
            PredicateType::AttributeRef => {
                is_left_attr_ref = true;
                attr_ref_exprs.push(
                    AttributeRefPred::from_pred_node(uncasted_left)
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
            PredicateType::AttributeRef => {
                attr_ref_exprs.push(
                    AttributeRefPred::from_pred_node(uncasted_right)
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
