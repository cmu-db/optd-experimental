//! Definition of physical expressions / operators in our query optimization framework.
//!
//! FIXME: All fields are placeholders.
//!
//! TODO Remove dead code.
//! TODO Figure out if each operator should be in a different submodule.
//! TODO This entire file is a WIP.

#![allow(dead_code)]

use crate::{entities::physical_expression::Model, memo::GroupId};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An interface defining what an in-memory physical expression representation should be able to do.
pub trait PhysicalExpression: From<Model> + Into<Model> + Clone + Debug {
    /// Returns the kind of relation / operator node encoded as an integer.
    fn kind(&self) -> i16;

    /// Retrieves the child groups IDs of this logical expression.
    fn children(&self) -> Vec<GroupId>;
}

impl PhysicalExpression for DefaultPhysicalExpression {
    fn kind(&self) -> i16 {
        match self {
            Self::TableScan(_) => 0,
            Self::Filter(_) => 1,
            Self::HashJoin(_) => 2,
        }
    }

    fn children(&self) -> Vec<GroupId> {
        match self {
            Self::TableScan(_) => vec![],
            Self::Filter(filter) => vec![filter.child],
            Self::HashJoin(hash_join) => vec![hash_join.left, hash_join.right],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefaultPhysicalExpression {
    TableScan(TableScan),
    Filter(PhysicalFilter),
    HashJoin(HashJoin),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TableScan {
    table_schema: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PhysicalFilter {
    child: GroupId,
    expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HashJoin {
    left: GroupId,
    right: GroupId,
    expression: String,
}

impl From<Model> for DefaultPhysicalExpression {
    fn from(value: Model) -> Self {
        match value.kind {
            0 => Self::TableScan(
                serde_json::from_value(value.data)
                    .expect("unable to deserialize data into a physical `TableScan`"),
            ),
            1 => Self::Filter(
                serde_json::from_value(value.data)
                    .expect("Unable to deserialize data into a physical `Filter`"),
            ),
            2 => Self::HashJoin(
                serde_json::from_value(value.data)
                    .expect("Unable to deserialize data into a physical `HashJoin`"),
            ),
            _ => panic!(),
        }
    }
}

impl From<DefaultPhysicalExpression> for Model {
    fn from(value: DefaultPhysicalExpression) -> Model {
        fn create_physical_expression(kind: i16, data: serde_json::Value) -> Model {
            Model {
                id: -1,
                group_id: -1,
                kind,
                data,
            }
        }

        match value {
            DefaultPhysicalExpression::TableScan(scan) => create_physical_expression(
                0,
                serde_json::to_value(scan).expect("unable to serialize physical `TableScan`"),
            ),
            DefaultPhysicalExpression::Filter(filter) => create_physical_expression(
                1,
                serde_json::to_value(filter).expect("unable to serialize physical `Filter`"),
            ),
            DefaultPhysicalExpression::HashJoin(join) => create_physical_expression(
                2,
                serde_json::to_value(join).expect("unable to serialize physical `HashJoin`"),
            ),
        }
    }
}

#[cfg(test)]
pub use build::*;

#[cfg(test)]
mod build {
    use super::*;
    use crate::expression::DefaultPhysicalExpression;

    pub fn table_scan(table_schema: String) -> DefaultPhysicalExpression {
        DefaultPhysicalExpression::TableScan(TableScan { table_schema })
    }
}
