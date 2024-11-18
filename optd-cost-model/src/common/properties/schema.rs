use itertools::Itertools;

use serde::{Deserialize, Serialize};

use super::Attribute;

/// [`Schema`] represents the schema of a group in the memo. It contains a list of attributes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub attributes: Vec<Attribute>,
}

impl std::fmt::Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.attributes.iter().map(|x| x.to_string()).join(", ")
        )
    }
}

impl Schema {
    pub fn new(attributes: Vec<Attribute>) -> Self {
        Self { attributes }
    }

    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
