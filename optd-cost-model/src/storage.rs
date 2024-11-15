#![allow(unused_variables)]
use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{Attr, StatType},
    BackendManager, CostModelStorageLayer,
};

use crate::{
    common::types::TableId, stats::AttributeCombValueStats, CostModelError, CostModelResult,
    SemanticError,
};

/// TODO: documentation
pub struct CostModelStorageManager<S: CostModelStorageLayer> {
    pub backend_manager: Arc<S>,
    // TODO: in-memory cache
}

impl<S: CostModelStorageLayer> CostModelStorageManager<S> {
    pub fn new(backend_manager: Arc<S>) -> Self {
        Self { backend_manager }
    }

    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&Field)
    pub async fn get_attribute_info(
        &self,
        table_id: TableId,
        attr_base_index: i32,
    ) -> CostModelResult<Attr> {
        let attr = self
            .backend_manager
            .get_attribute(table_id.into(), attr_base_index)
            .await?;
        attr.ok_or_else(|| {
            CostModelError::SemanticError(SemanticError::AttributeNotFound(
                table_id,
                attr_base_index,
            ))
        })
    }

    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    pub async fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_base_indices: &[i32],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        Ok(self
            .backend_manager
            .get_stats_for_attr_indices_based(
                table_id.into(),
                attr_base_indices.to_vec(),
                StatType::Comb,
                None,
            )
            .await?
            .map(|json| json.into()))
    }
}
