use serde::{Deserialize, Serialize};

use crate::{
    common::{predicates::constant_pred::ConstantType, types::TableId},
    stats::AttributeCombValueStats,
    CostModelResult,
};

pub mod mock;
pub mod persistent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub typ: ConstantType,
    pub nullable: bool,
}

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
        attr_base_indices: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>>;
}
