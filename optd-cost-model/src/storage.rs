#![allow(unused_variables)]
use std::sync::Arc;

use optd_persistent::{cost_model::interface::Attr, BackendManager, CostModelStorageLayer};

use crate::{
    common::types::TableId, stats::AttributeCombValueStats, CostModelError, CostModelResult,
    SemanticError,
};

/// TODO: documentation
pub struct CostModelStorageManager<S: CostModelStorageLayer> {
    pub backend_manager: Arc<S>,
    // TODO: in-memory cache
}

impl CostModelStorageManager<BackendManager> {
    pub fn new(backend_manager: Arc<BackendManager>) -> Self {
        Self { backend_manager }
    }

    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&Field)
    pub async fn get_attribute_info(
        &self,
        table_id: TableId,
        attribute_base_index: i32,
    ) -> CostModelResult<Attr> {
        let attr = self
            .backend_manager
            .get_attribute(table_id.into(), attribute_base_index)
            .await?;
        attr.ok_or_else(|| {
            CostModelError::SemanticError(SemanticError::AttributeNotFound(
                table_id,
                attribute_base_index,
            ))
        })
    }

    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    pub fn get_attributes_comb_statistics(
        &self,
        table_id: TableId,
        attr_comb: &[usize],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        todo!()
    }
}
