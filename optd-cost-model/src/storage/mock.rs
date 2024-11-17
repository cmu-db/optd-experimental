#![allow(unused_variables, dead_code)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{common::types::TableId, stats::AttributeCombValueStats, CostModelResult};

use super::{Attribute, CostModelStorageManager};

pub type AttrIndices = Vec<u64>;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct TableStats {
    pub row_cnt: usize,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub column_comb_stats: HashMap<AttrIndices, AttributeCombValueStats>,
}

impl TableStats {
    pub fn new(
        row_cnt: usize,
        column_comb_stats: HashMap<AttrIndices, AttributeCombValueStats>,
    ) -> Self {
        Self {
            row_cnt,
            column_comb_stats,
        }
    }
}

pub type BaseTableStats = HashMap<TableId, TableStats>;
pub type BaseTableAttrInfo = HashMap<TableId, HashMap<u64, Attribute>>; // (table_id, (attr_base_index, attr))

pub struct CostModelStorageMockManagerImpl {
    pub(crate) per_table_stats_map: BaseTableStats,
    pub(crate) per_table_attr_infos_map: BaseTableAttrInfo,
}

impl CostModelStorageMockManagerImpl {
    pub fn new(
        per_table_stats_map: BaseTableStats,
        per_table_attr_infos_map: BaseTableAttrInfo,
    ) -> Self {
        Self {
            per_table_stats_map,
            per_table_attr_infos_map,
        }
    }
}

impl CostModelStorageManager for CostModelStorageMockManagerImpl {
    async fn get_attribute_info(
        &self,
        table_id: TableId,
        attr_base_index: i32,
    ) -> CostModelResult<Option<Attribute>> {
        let table_attr_infos = self.per_table_attr_infos_map.get(&table_id);
        match table_attr_infos {
            None => Ok(None),
            Some(table_attr_infos) => match table_attr_infos.get(&(attr_base_index as u64)) {
                None => Ok(None),
                Some(attr) => Ok(Some(attr.clone())),
            },
        }
    }

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
}
