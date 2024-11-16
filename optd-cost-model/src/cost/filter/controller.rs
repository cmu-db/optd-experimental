use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{in_list_pred::InListPred, like_pred::LikePred, un_op_pred::UnOpType},
    },
    cost_model::CostModelImpl,
    stats::UNIMPLEMENTED_SEL,
    storage::CostModelStorageManager,
    CostModelResult, EstimatedStatistic,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    // TODO: is it a good design to pass table_id here? I think it needs to be refactored.
    // Consider to remove table_id.
    pub async fn get_filter_row_cnt(
        &self,
        child_row_cnt: EstimatedStatistic,
        cond: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let selectivity = { self.get_filter_selectivity(cond).await? };
        Ok(
            EstimatedStatistic((child_row_cnt.0 as f64 * selectivity) as u64)
                .max(EstimatedStatistic(1)),
        )
    }

    pub async fn get_filter_selectivity(
        &self,
        expr_tree: ArcPredicateNode,
    ) -> CostModelResult<f64> {
        Box::pin(async move {
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
                        UnOpType::Not => Ok(1.0 - self.get_filter_selectivity(child).await?),
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
                        self.get_comp_op_selectivity(*bin_op_typ, left_child, right_child).await
                    } else if bin_op_typ.is_numerical() {
                        panic!(
                            "the selectivity of operations that return numerical values is undefined"
                        )
                    } else {
                        unreachable!("all BinOpTypes should be true for at least one is_*() function")
                    }
                }
                PredicateType::LogOp(log_op_typ) => {
                    self.get_log_op_selectivity(*log_op_typ, &expr_tree.children).await
                }
                PredicateType::Func(_) => unimplemented!("check bool type or else panic"),
                PredicateType::SortOrder(_) => {
                    panic!("the selectivity of sort order expressions is undefined")
                }
                PredicateType::Between => Ok(UNIMPLEMENTED_SEL),
                PredicateType::Cast => unimplemented!("check bool type or else panic"),
                PredicateType::Like => {
                    let like_expr = LikePred::from_pred_node(expr_tree).unwrap();
                    self.get_like_selectivity(&like_expr).await
                }
                PredicateType::DataType(_) => {
                    panic!("the selectivity of a data type is not defined")
                }
                PredicateType::InList => {
                    let in_list_expr = InListPred::from_pred_node(expr_tree).unwrap();
                    self.get_in_list_selectivity(&in_list_expr).await
                }
                _ => unreachable!(
                    "all expression DfPredType were enumerated. this should be unreachable"
                ),
            }
        }).await
    }
}
