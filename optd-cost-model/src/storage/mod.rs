use crate::{common::types::TableId, stats::AttributeCombValueStats, CostModelResult};

pub mod mock;
pub mod persistent;

#[trait_variant::make(Send)]
pub trait CostModelStorageManager {
    async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>>;

    async fn get_table_row_count(&self, table_id: TableId) -> CostModelResult<Option<u64>>;
}
