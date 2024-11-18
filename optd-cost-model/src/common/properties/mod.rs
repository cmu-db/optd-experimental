use serde::{Deserialize, Serialize};

use super::predicates::constant_pred::ConstantType;

pub mod attr_ref;
pub mod schema;

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
