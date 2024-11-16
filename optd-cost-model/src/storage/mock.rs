#![allow(unused_variables, dead_code)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{common::types::TableId, stats::AttributeCombValueStats, CostModelResult};

use super::{Attribute, CostModelStorageManager};

pub type AttrsIdx = Vec<usize>;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct TableStats {
    pub row_cnt: usize,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub column_comb_stats: HashMap<AttrsIdx, AttributeCombValueStats>,
}

impl TableStats {
    pub fn new(
        row_cnt: usize,
        column_comb_stats: HashMap<AttrsIdx, AttributeCombValueStats>,
    ) -> Self {
        Self {
            row_cnt,
            column_comb_stats,
        }
    }
}

pub type BaseTableStats = HashMap<String, TableStats>;

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
    async fn get_attribute_info(
        &self,
        table_id: TableId,
        attr_base_index: i32,
    ) -> CostModelResult<Option<Attribute>> {
        todo!()
    }

    async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[usize],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        todo!()
    }
}
