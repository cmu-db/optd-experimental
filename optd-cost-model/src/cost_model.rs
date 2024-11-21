#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{CatalogSource, Stat, StatType},
    CostModelStorageLayer,
};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PhysicalNodeType, ReprPredicateNode},
        predicates::list_pred::ListPred,
        types::{AttrId, EpochId, ExprId, TableId},
    },
    memo_ext::MemoExt,
    stats::AttributeCombValueStats,
    storage::{self, CostModelStorageManager},
    ComputeCostContext, Cost, CostModel, CostModelResult, EstimatedStatistic, StatValue,
};

/// TODO: documentation
pub struct CostModelImpl<S: CostModelStorageManager> {
    pub storage_manager: S,
    pub default_catalog_source: CatalogSource,
    pub memo: Arc<dyn MemoExt>,
}

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// TODO: documentation
    pub fn new(
        storage_manager: S,
        default_catalog_source: CatalogSource,
        memo: Arc<dyn MemoExt>,
    ) -> Self {
        Self {
            storage_manager,
            default_catalog_source,
            memo,
        }
    }
}

#[async_trait::async_trait]
impl<S: CostModelStorageManager + Send + Sync + 'static> CostModel for CostModelImpl<S> {
    /// TODO: should we add epoch_id?
    async fn compute_operation_cost(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_costs: &[Cost],
        children_stats: &[EstimatedStatistic],
        context: ComputeCostContext,
    ) -> CostModelResult<Cost> {
        let res = self.storage_manager.get_cost(context.expr_id).await;
        if let Ok((Some(cost), _)) = res {
            return Ok(cost);
        };
        let mut output_statistic = None;
        if let Ok((_, Some(statistic))) = res {
            output_statistic = Some(statistic);
        };
        let output_cost = match node {
            PhysicalNodeType::PhysicalScan => {
                let output_statistic_data = output_statistic.unwrap_or(
                    self.derive_statistics(
                        node,
                        predicates,
                        children_stats,
                        context.clone(),
                        false,
                    )
                    .await?,
                );
                output_statistic = Some(output_statistic_data.clone());
                Cost {
                    compute_cost: 0.0,
                    io_cost: output_statistic_data.0,
                }
            }
            PhysicalNodeType::PhysicalEmptyRelation => Cost {
                compute_cost: 0.1,
                io_cost: 0.0,
            },
            PhysicalNodeType::PhysicalLimit => Cost {
                compute_cost: children_costs[0].compute_cost,
                io_cost: 0.0,
            },
            PhysicalNodeType::PhysicalFilter => Cost {
                // TODO: now this equation is specific to optd, and try to make this equation more general
                compute_cost: children_costs[1].compute_cost * children_stats[0].0,
                io_cost: 0.0,
            },
            PhysicalNodeType::PhysicalNestedLoopJoin(join_typ) => {
                let child_compute_cost = children_costs[2].compute_cost;
                Cost {
                    compute_cost: children_stats[0].0 * children_stats[1].0 * child_compute_cost
                        + children_stats[0].0,
                    io_cost: 0.0,
                }
            }
            // TODO: we should document that the first child is the left table, which is used to build
            // the hash table.
            PhysicalNodeType::PhysicalHashJoin(join_typ) => Cost {
                compute_cost: children_stats[0].0 * 2.0 + children_stats[1].0,
                io_cost: 0.0,
            },
            PhysicalNodeType::PhysicalAgg => Cost {
                compute_cost: children_stats[0].0
                    * (children_costs[1].compute_cost + children_costs[2].compute_cost),
                io_cost: 0.0,
            },
            PhysicalNodeType::PhysicalProjection => Cost {
                compute_cost: children_stats[0].0 * children_costs[1].compute_cost,
                io_cost: 0.0,
            },
            PhysicalNodeType::PhysicalSort => Cost {
                compute_cost: children_stats[0].0 * children_stats[0].0.ln_1p().max(1.0),
                io_cost: 0.0,
            },
        };
        let res = self
            .storage_manager
            .store_cost(
                context.expr_id,
                Some(output_cost.clone()),
                output_statistic,
                None,
            )
            .await;
        if res.is_err() {
            eprintln!("Failed to store output cost");
        }
        Ok(output_cost)
    }

    /// TODO: should we add epoch_id?
    async fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[EstimatedStatistic],
        context: ComputeCostContext,
        store_output_statistic: bool,
    ) -> CostModelResult<EstimatedStatistic> {
        let res = self.storage_manager.get_cost(context.expr_id).await;
        if let Ok((_, Some(statistic))) = res {
            return Ok(statistic);
        }
        let output_statistic = match node {
            PhysicalNodeType::PhysicalScan => {
                let table_id = TableId(predicates[0].data.as_ref().unwrap().as_u64());
                let row_cnt = self
                    .storage_manager
                    .get_table_row_count(table_id)
                    .await?
                    .unwrap_or(1) as f64;
                Ok(EstimatedStatistic(row_cnt))
            }
            PhysicalNodeType::PhysicalEmptyRelation => Ok(EstimatedStatistic(0.01)),
            PhysicalNodeType::PhysicalLimit => {
                self.get_limit_row_cnt(children_statistics[0].clone(), predicates[1].clone())
            }
            PhysicalNodeType::PhysicalFilter => {
                self.get_filter_row_cnt(
                    children_statistics[0].clone(),
                    context.group_id,
                    predicates[0].clone(),
                )
                .await
            }
            PhysicalNodeType::PhysicalNestedLoopJoin(join_typ) => {
                self.get_nlj_row_cnt(
                    join_typ,
                    context.group_id,
                    children_statistics[0].clone(),
                    children_statistics[1].clone(),
                    context.children_group_ids[0],
                    context.children_group_ids[1],
                    predicates[0].clone(),
                )
                .await
            }
            PhysicalNodeType::PhysicalHashJoin(join_typ) => {
                self.get_hash_join_row_cnt(
                    join_typ,
                    context.group_id,
                    children_statistics[0].clone(),
                    children_statistics[1].clone(),
                    context.children_group_ids[0],
                    context.children_group_ids[1],
                    ListPred::from_pred_node(predicates[0].clone()).unwrap(),
                    ListPred::from_pred_node(predicates[1].clone()).unwrap(),
                )
                .await
            }
            PhysicalNodeType::PhysicalAgg => {
                self.get_agg_row_cnt(context.group_id, predicates[1].clone())
                    .await
            }
            PhysicalNodeType::PhysicalSort | PhysicalNodeType::PhysicalProjection => {
                Ok(children_statistics[0].clone())
            }
        }?;
        if store_output_statistic {
            let res = self
                .storage_manager
                .store_cost(context.expr_id, None, Some(output_statistic.clone()), None)
                .await;
            if res.is_err() {
                eprintln!("Failed to store output statistic");
            }
        };
        Ok(output_statistic)
    }

    async fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()> {
        todo!()
    }

    async fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    async fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    async fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<crate::Cost>> {
        todo!()
    }
}

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    pub(crate) async fn get_attribute_comb_stats(
        &self,
        table_id: TableId,
        attr_comb: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        self.storage_manager
            .get_attributes_comb_statistics(table_id, attr_comb)
            .await
    }
}

// TODO: Add tests for `derive_statistic`` and `compute_operation_cost`.
