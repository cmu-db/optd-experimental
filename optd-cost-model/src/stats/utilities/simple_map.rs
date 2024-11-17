use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::values::Value;

/// TODO: documentation
/// Now it is mainly for testing purposes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SimpleMap {
    pub(crate) m: HashMap<Value, f64>,
}

impl SimpleMap {
    pub fn new(v: Vec<(Value, f64)>) -> Self {
        Self {
            m: v.into_iter().collect(),
        }
    }
}
