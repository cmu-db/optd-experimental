use std::sync::Arc;

use optd_persistent::CostModelStorageLayer;

pub struct CostModelStorageManager<CMSL: CostModelStorageLayer> {
    pub backend_manager: Arc<CMSL>,
    // TODO: in-memory cache
}

impl<CMSL: CostModelStorageLayer> CostModelStorageManager<CMSL> {
    pub fn new(backend_manager: Arc<CMSL>) -> Self {
        Self { backend_manager }
    }
}
