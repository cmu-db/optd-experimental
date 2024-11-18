use serde::{Deserialize, Serialize};

use super::predicates::constant_pred::ConstantType;

pub mod attr_ref;
pub mod schema;

const DEFAULT_NAME: &str = "unnamed";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub typ: ConstantType,
    pub nullable: bool,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.nullable {
            write!(f, "{}:{:?}", self.name, self.typ)
        } else {
            write!(f, "{}:{:?}(non-null)", self.name, self.typ)
        }
    }
}

impl Attribute {
    /// Generate a field that is only a place holder whose members are never used.
    fn placeholder() -> Self {
        Self {
            name: DEFAULT_NAME.to_string(),
            typ: ConstantType::Binary,
            nullable: true,
        }
    }
}
