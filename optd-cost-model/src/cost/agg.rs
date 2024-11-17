use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{attr_ref_pred::AttributeRefPred, list_pred::ListPred},
        types::TableId,
    },
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    storage::CostModelStorageManager,
    CostModelError, CostModelResult, EstimatedStatistic, SemanticError,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    pub async fn get_agg_row_cnt(
        &self,
        group_by: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let group_by = ListPred::from_pred_node(group_by).unwrap();
        if group_by.is_empty() {
            Ok(EstimatedStatistic(1))
        } else {
            // Multiply the n-distinct of all the group by columns.
            // TODO: improve with multi-dimensional n-distinct
            let mut row_cnt = 1;

            for node in &group_by.0.children {
                match node.typ {
                    PredicateType::AttributeRef => {
                        let attr_ref =
                            AttributeRefPred::from_pred_node(node.clone()).ok_or_else(|| {
                                SemanticError::InvalidPredicate(
                                    "Expected AttributeRef predicate".to_string(),
                                )
                            })?;
                        if attr_ref.is_derived() {
                            row_cnt *= DEFAULT_NUM_DISTINCT;
                        } else {
                            let table_id = attr_ref.table_id();
                            let attr_idx = attr_ref.attr_index();
                            // TODO: Only query ndistinct instead of all kinds of stats.
                            let stats_option =
                                self.get_attribute_comb_stats(table_id, &[attr_idx]).await?;

                            let ndistinct = match stats_option {
                                Some(stats) => stats.ndistinct,
                                None => {
                                    // The column type is not supported or stats are missing.
                                    DEFAULT_NUM_DISTINCT
                                }
                            };
                            row_cnt *= ndistinct;
                        }
                    }
                    _ => {
                        // TODO: Consider the case where `GROUP BY 1`.
                        panic!("GROUP BY must have attribute ref predicate");
                    }
                }
            }
            Ok(EstimatedStatistic(row_cnt))
        }
    }
}
