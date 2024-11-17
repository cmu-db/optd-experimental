use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::common::values::Value;

/// TODO: documentation
/// Now it is mainly for testing purposes.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SimpleMap<K: Eq + Hash + Clone> {
    pub(crate) m: HashMap<K, f64>,
}

impl<K: Eq + Hash + Clone> SimpleMap<K> {
    pub fn new(v: Vec<(K, f64)>) -> Self {
        Self {
            m: v.into_iter().collect(),
        }
    }
}
