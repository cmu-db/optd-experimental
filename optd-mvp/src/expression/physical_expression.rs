//! Definition of physical expressions / operators in the Cascades query optimization framework.
//!
//! FIXME: All fields are placeholders.
//!
//! TODO Remove dead code.
//! TODO Figure out if each operator should be in a different submodule.
//! TODO This entire file is a WIP.

#![allow(dead_code)]

use crate::{entities::*, memo::GroupId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhysicalExpression {
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

/// TODO Use a macro instead.
impl From<physical_expression::Model> for PhysicalExpression {
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

/// TODO Use a macro instead.
impl From<PhysicalExpression> for physical_expression::Model {
    fn from(value: PhysicalExpression) -> physical_expression::Model {
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
            PhysicalExpression::TableScan(scan) => create_physical_expression(
                0,
                serde_json::to_value(scan).expect("unable to serialize physical `TableScan`"),
            ),
            PhysicalExpression::Filter(filter) => create_physical_expression(
                1,
                serde_json::to_value(filter).expect("unable to serialize physical `Filter`"),
            ),
            PhysicalExpression::HashJoin(join) => create_physical_expression(
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
    use crate::expression::PhysicalExpression;

    pub fn table_scan(table_schema: String) -> PhysicalExpression {
        PhysicalExpression::TableScan(TableScan { table_schema })
    }

    pub fn filter(child_group: GroupId, expression: String) -> PhysicalExpression {
        PhysicalExpression::Filter(PhysicalFilter {
            child: child_group,
            expression,
        })
    }

    pub fn hash_join(
        left_group: GroupId,
        right_group: GroupId,
        expression: String,
    ) -> PhysicalExpression {
        PhysicalExpression::HashJoin(HashJoin {
            left: left_group,
            right: right_group,
            expression,
        })
    }
}
