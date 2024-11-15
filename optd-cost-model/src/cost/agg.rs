use optd_persistent::CostModelStorageLayer;

use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{attr_ref_pred::AttributeRefPred, list_pred::ListPred},
        types::TableId,
    },
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    CostModelError, CostModelResult, EstimatedStatistic,
};

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    pub fn get_agg_row_cnt(
        &self,
        group_by: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let group_by = ListPred::from_pred_node(group_by).unwrap();
        if group_by.is_empty() {
            Ok(EstimatedStatistic(1))
        } else {
            // Multiply the n-distinct of all the group by columns.
            // TODO: improve with multi-dimensional n-distinct
            let row_cnt = group_by.0.children.iter().try_fold(1, |acc, node| {
                match node.typ {
                    PredicateType::AttributeRef => {
                        let attr_ref =
                            AttributeRefPred::from_pred_node(node.clone()).ok_or_else(|| {
                                CostModelError::InvalidPredicate(
                                    "Expected AttributeRef predicate".to_string(),
                                )
                            })?;
                        if attr_ref.is_derived() {
                            Ok(acc * DEFAULT_NUM_DISTINCT)
                        } else {
                            let table_id = attr_ref.table_id();
                            let attr_idx = attr_ref.attr_index();
                            let stats_option =
                                self.get_attribute_comb_stats(TableId(table_id), &[attr_idx])?;

                            let ndistinct = match stats_option {
                                Some(stats) => stats.ndistinct,
                                None => {
                                    // The column type is not supported or stats are missing.
                                    DEFAULT_NUM_DISTINCT
                                }
                            };
                            Ok(acc * ndistinct)
                        }
                    }
                    _ => {
                        // TODO: Consider the case where `GROUP BY 1`.
                        panic!("GROUP BY must have attribute ref predicate")
                    }
                }
            })?;
            Ok(EstimatedStatistic(row_cnt))
        }
    }
}
