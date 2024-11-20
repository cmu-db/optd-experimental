use crate::{
    common::types::{EpochId, ExprId, TableId},
    stats::AttributeCombValueStats,
    Cost, CostModelResult, EstimatedStatistic,
};

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

    async fn get_cost(
        &self,
        expr_id: ExprId,
    ) -> CostModelResult<(Option<Cost>, Option<EstimatedStatistic>)>;

    async fn store_cost(
        &self,
        expr_id: ExprId,
        cost: Option<Cost>,
        estimated_statistic: Option<EstimatedStatistic>,
        epoch_id: EpochId,
    ) -> CostModelResult<()>;
}
