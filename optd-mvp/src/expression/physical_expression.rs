//! Definition of physical expressions / operators in the Cascades query optimization framework.
//!
//! FIXME: All fields are placeholders, and group IDs are just represented as i32 for now.
//!
//! TODO figure out if each operator should be in a different submodule.
//! TODO This entire file is a WIP.

use crate::entities::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum PhysicalExpression {
    TableScan(TableScan),
    Filter(PhysicalFilter),
    HashJoin(HashJoin),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableScan {
    table_schema: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhysicalFilter {
    child: i32,
    expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HashJoin {
    left: i32,
    right: i32,
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
    use crate::expression::Expression;

    pub fn table_scan(table_schema: String) -> Expression {
        Expression::Physical(PhysicalExpression::TableScan(TableScan { table_schema }))
    }

    pub fn filter(child_group: i32, expression: String) -> Expression {
        Expression::Physical(PhysicalExpression::Filter(PhysicalFilter {
            child: child_group,
            expression,
        }))
    }

    pub fn hash_join(left_group: i32, right_group: i32, expression: String) -> Expression {
        Expression::Physical(PhysicalExpression::HashJoin(HashJoin {
            left: left_group,
            right: right_group,
            expression,
        }))
    }
}
