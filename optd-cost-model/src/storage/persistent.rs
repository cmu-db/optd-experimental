#![allow(unused_variables)]
use std::sync::Arc;

use optd_persistent::{cost_model::interface::StatType, CostModelStorageLayer};

use crate::{
    common::types::TableId,
    stats::{utilities::counter::Counter, AttributeCombValueStats, Distribution, MostCommonValues},
    CostModelResult,
};

use super::CostModelStorageManager;

/// TODO: documentation
pub struct CostModelStorageManagerImpl<S: CostModelStorageLayer + Send + Sync> {
    pub backend_manager: Arc<S>,
    // TODO: in-memory cache
}

impl<S: CostModelStorageLayer + Send + Sync> CostModelStorageManagerImpl<S> {
    pub fn new(backend_manager: Arc<S>) -> Self {
        Self { backend_manager }
    }
}

impl<S: CostModelStorageLayer + Send + Sync> CostModelStorageManager
    for CostModelStorageManagerImpl<S>
{
    /// Gets the latest statistics for a given table. Currently we only support base table
    /// statistic retrieval.
    ///
    /// TODO: Currently, in `AttributeCombValueStats`, only `Distribution` is optional.
    /// This poses a question about the behavior of the system if there is no corresponding
    /// `MostCommonValues`, `ndistinct`, or other statistics. We should have a clear
    /// specification about the behavior of the system in the presence of missing statistics.
    ///
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    ///
    /// TODO: Shall we pass in an epoch here to make sure that the statistics are from the same
    /// epoch?
    async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[u64],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        let dist: Option<Distribution> = self
            .backend_manager
            .get_stats_for_attr_indices_based(
                table_id.into(),
                attr_base_indices.iter().map(|&x| x as i32).collect(),
                StatType::Distribution,
                None,
            )
            .await?
            .map(|json| serde_json::from_value(json).unwrap());

        let mcvs = self
            .backend_manager
            .get_stats_for_attr_indices_based(
                table_id.into(),
                attr_base_indices.iter().map(|&x| x as i32).collect(),
                StatType::MostCommonValues,
                None,
            )
            .await?
            .map(|json| serde_json::from_value(json).unwrap())
            .unwrap_or_else(|| MostCommonValues::Counter(Counter::default()));

        let ndistinct = self
            .backend_manager
            .get_stats_for_attr_indices_based(
                table_id.into(),
                attr_base_indices.iter().map(|&x| x as i32).collect(),
                StatType::Cardinality,
                None,
            )
            .await?
            .map(|json| serde_json::from_value(json).unwrap())
            .unwrap_or(0);

        let table_row_count = self
            .backend_manager
            .get_stats_for_attr_indices_based(
                table_id.into(),
                attr_base_indices.iter().map(|&x| x as i32).collect(),
                StatType::TableRowCount,
                None,
            )
            .await?
            .map(|json| serde_json::from_value(json).unwrap())
            .unwrap_or(0);
        let non_null_count = self
            .backend_manager
            .get_stats_for_attr_indices_based(
                table_id.into(),
                attr_base_indices.iter().map(|&x| x as i32).collect(),
                StatType::NonNullCount,
                None,
            )
            .await?
            .map(|json| serde_json::from_value(json).unwrap())
            .unwrap_or(0);

        // FIXME: Only minimal checks for invalid values is conducted here. We should have
        // much clear specification about the behavior of the system in the presence of
        // invalid statistics.
        let null_frac = if table_row_count == 0 {
            0.0
        } else {
            1.0 - (non_null_count as f64 / table_row_count as f64)
        };

        Ok(Some(AttributeCombValueStats::new(
            mcvs, dist, ndistinct, null_frac,
        )))
    }

    // TODO: Support querying for a specific type of statistics.
}
