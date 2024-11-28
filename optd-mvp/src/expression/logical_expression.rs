//! Definition of logical expressions / relations in the Cascades query optimization framework.
//!
//! FIXME: All fields are placeholders, and group IDs are just represented as i32 for now.
//! FIXME: Representation needs to know how to "rewrite" child group IDs to whatever a fingerprint
//! will need.
//!
//! TODO figure out if each relation should be in a different submodule.
//! TODO This entire file is a WIP.

use crate::entities::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum LogicalExpression {
    Scan(Scan),
    Filter(Filter),
    Join(Join),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scan {
    table_schema: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filter {
    child: i32,
    expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Join {
    left: i32,
    right: i32,
    expression: String,
}

/// TODO Use a macro instead.
impl From<logical_expression::Model> for LogicalExpression {
    fn from(value: logical_expression::Model) -> Self {
        match value.kind {
            0 => Self::Scan(
                serde_json::from_value(value.data)
                    .expect("unable to deserialize data into a logical `Scan`"),
            ),
            1 => Self::Filter(
                serde_json::from_value(value.data)
                    .expect("Unable to deserialize data into a logical `Filter`"),
            ),
            2 => Self::Join(
                serde_json::from_value(value.data)
                    .expect("Unable to deserialize data into a logical `Join`"),
            ),
            _ => panic!(),
        }
    }
}

/// TODO Use a macro instead.
impl From<LogicalExpression> for logical_expression::Model {
    fn from(value: LogicalExpression) -> logical_expression::Model {
        fn create_logical_expression(
            kind: i16,
            data: serde_json::Value,
        ) -> logical_expression::Model {
            logical_expression::Model {
                id: -1,
                group_id: -1,
                kind,
                data,
            }
        }

        match value {
            LogicalExpression::Scan(scan) => create_logical_expression(
                0,
                serde_json::to_value(scan).expect("unable to serialize logical `Scan`"),
            ),
            LogicalExpression::Filter(filter) => create_logical_expression(
                1,
                serde_json::to_value(filter).expect("unable to serialize logical `Filter`"),
            ),
            LogicalExpression::Join(join) => create_logical_expression(
                2,
                serde_json::to_value(join).expect("unable to serialize logical `Join`"),
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

    pub fn scan(table_schema: String) -> Expression {
        Expression::Logical(LogicalExpression::Scan(Scan { table_schema }))
    }

    pub fn filter(child_group: i32, expression: String) -> Expression {
        Expression::Logical(LogicalExpression::Filter(Filter {
            child: child_group,
            expression,
        }))
    }

    pub fn join(left_group: i32, right_group: i32, expression: String) -> Expression {
        Expression::Logical(LogicalExpression::Join(Join {
            left: left_group,
            right: right_group,
            expression,
        }))
    }
}
