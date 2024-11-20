#![allow(unused_variables, dead_code)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::types::{EpochId, ExprId, TableId},
    stats::AttributeCombValueStats,
    Cost, CostModelResult, EstimatedStatistic,
};

use super::CostModelStorageManager;

pub type AttrIndices = Vec<u64>;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct TableStats {
    pub row_cnt: u64,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub column_comb_stats: HashMap<AttrIndices, AttributeCombValueStats>,
}

impl TableStats {
    pub fn new(
        row_cnt: u64,
        column_comb_stats: HashMap<AttrIndices, AttributeCombValueStats>,
    ) -> Self {
        Self {
            row_cnt,
            column_comb_stats,
        }
    }
}

pub type BaseTableStats = HashMap<TableId, TableStats>;

pub struct CostModelStorageMockManagerImpl {
    pub(crate) per_table_stats_map: BaseTableStats,
}

impl CostModelStorageMockManagerImpl {
    pub fn new(per_table_stats_map: BaseTableStats) -> Self {
        Self {
            per_table_stats_map,
        }
    }
}

impl CostModelStorageManager for CostModelStorageMockManagerImpl {
    async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        let table_stats = self.per_table_stats_map.get(&table_id);
        match table_stats {
            None => Ok(None),
            Some(table_stats) => match table_stats.column_comb_stats.get(attr_base_indices) {
                None => Ok(None),
                Some(stats) => Ok(Some(stats.clone())),
            },
        }
    }

    async fn get_table_row_count(&self, table_id: TableId) -> CostModelResult<Option<u64>> {
        let table_stats = self.per_table_stats_map.get(&table_id);
        Ok(table_stats.map(|stats| stats.row_cnt))
    }

    /// TODO: finish this when implementing the cost get/store tests
    async fn get_cost(
        &self,
        expr_id: ExprId,
    ) -> CostModelResult<(Option<crate::Cost>, Option<EstimatedStatistic>)> {
        todo!()
    }

    /// TODO: finish this when implementing the cost get/store tests
    async fn store_cost(
        &self,
        expr_id: ExprId,
        cost: Option<Cost>,
        estimated_statistic: Option<EstimatedStatistic>,
        epoch_id: EpochId,
    ) -> CostModelResult<()> {
        todo!()
    }
}
