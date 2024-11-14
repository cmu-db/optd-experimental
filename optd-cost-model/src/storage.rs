use std::sync::Arc;

use optd_persistent::CostModelStorageLayer;

/// TODO: documentation
pub struct CostModelStorageManager<S: CostModelStorageLayer> {
    pub backend_manager: Arc<S>,
    // TODO: in-memory cache
}

impl<S: CostModelStorageLayer> CostModelStorageManager<S> {
    /// TODO: documentation
    pub fn new(backend_manager: Arc<S>) -> Self {
        Self { backend_manager }
    }
}
