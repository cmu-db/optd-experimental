#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{CatalogSource, Stat, StatType},
    CostModelStorageLayer,
};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PhysicalNodeType},
        types::{AttrId, EpochId, ExprId, TableId},
    },
    stats::AttributeCombValueStats,
    storage::CostModelStorageManager,
    ComputeCostContext, Cost, CostModel, CostModelResult, EstimatedStatistic, StatValue,
};

/// TODO: documentation
pub struct CostModelImpl<S: CostModelStorageLayer> {
    pub storage_manager: CostModelStorageManager<S>,
    pub default_catalog_source: CatalogSource,
}

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    /// TODO: documentation
    pub fn new(
        storage_manager: CostModelStorageManager<S>,
        default_catalog_source: CatalogSource,
    ) -> Self {
        Self {
            storage_manager,
            default_catalog_source,
        }
    }
}

impl<S: CostModelStorageLayer + std::marker::Sync + 'static> CostModel for CostModelImpl<S> {
    fn compute_operation_cost(
        &self,
        node: &PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[Option<&EstimatedStatistic>],
        context: Option<ComputeCostContext>,
    ) -> CostModelResult<Cost> {
        todo!()
    }

    fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[Option<&EstimatedStatistic>],
        context: Option<ComputeCostContext>,
    ) -> CostModelResult<EstimatedStatistic> {
        todo!()
    }

    fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()> {
        todo!()
    }

    fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<crate::Cost>> {
        todo!()
    }
}

impl<S: CostModelStorageLayer> CostModelImpl<S> {
    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    pub(crate) fn get_attribute_comb_stats(
        &self,
        table_id: TableId,
        attr_comb: &[usize],
    ) -> CostModelResult<AttributeCombValueStats> {
        self.storage_manager
            .get_attributes_comb_statistics(table_id, attr_comb)
    }
}
