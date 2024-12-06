//! Definition of physical expressions / operators in our query optimization framework.
//!
//! FIXME: All fields are placeholders.
//!
//! TODO Figure out if each operator should be in a different submodule.
//! TODO This entire file is a WIP.

use crate::{entities::*, memo::GroupId};
use serde::{Deserialize, Serialize};

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

/// TODO Use a macro.
impl From<physical_expression::Model> for DefaultPhysicalExpression {
    fn from(value: physical_expression::Model) -> Self {
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

/// TODO Use a macro.
impl From<DefaultPhysicalExpression> for physical_expression::Model {
    fn from(value: DefaultPhysicalExpression) -> physical_expression::Model {
        fn create_physical_expression(
            kind: i16,
            data: serde_json::Value,
        ) -> physical_expression::Model {
            physical_expression::Model {
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
