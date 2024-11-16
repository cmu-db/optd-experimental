use persistent::Attribute;

use crate::{common::types::TableId, stats::AttributeCombValueStats, CostModelResult};

pub mod mock;
pub mod persistent;

#[trait_variant::make(Send)]
pub trait CostModelStorageManager {
    async fn get_attribute_info(
        &self,
        table_id: TableId,
        attr_base_index: i32,
    ) -> CostModelResult<Option<Attribute>>;

    async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[usize],
    ) -> CostModelResult<Option<AttributeCombValueStats>>;
}
