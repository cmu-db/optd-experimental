#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{CatalogSource, Stat, StatType},
    CostModelStorageLayer,
};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PhysicalNodeType},
        types::ExprId,
    },
    storage::CostModelStorageManager,
    ComputeCostContext, CostModel, CostModelResult, EstimatedStatistic,
};

pub struct CostModelImpl<CMSL: CostModelStorageLayer> {
    storage_manager: CostModelStorageManager<CMSL>,
    default_catalog_source: CatalogSource,
}

impl<CMSL: CostModelStorageLayer> CostModelImpl<CMSL> {
    pub fn new(
        storage_manager: CostModelStorageManager<CMSL>,
        default_catalog_source: CatalogSource,
    ) -> Self {
        Self {
            storage_manager,
            default_catalog_source,
        }
    }
}

impl<CMSL: CostModelStorageLayer + std::marker::Sync + 'static> CostModel for CostModelImpl<CMSL> {
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
        // TODO: i32 should be changed to TableId.
        table_id: i32,
        stat_type: StatType,
        epoch_id: Option<i32>,
    ) -> CostModelResult<Option<crate::StatValue>> {
        todo!()
    }

    fn get_attribute_statistic_for_analysis(
        &self,
        // TODO: i32 should be changed to AttrId or EpochId.
        attr_ids: Vec<i32>,
        stat_type: StatType,
        epoch_id: Option<i32>,
    ) -> CostModelResult<Option<crate::StatValue>> {
        todo!()
    }

    fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<i32>,
    ) -> CostModelResult<Option<crate::Cost>> {
        todo!()
    }
}
