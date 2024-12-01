//! Definition of logical expressions / relations in our query optimization framework.
//!
//! FIXME: All fields are placeholders.
//!
//! TODO Remove dead code.
//! TODO Figure out if each relation should be in a different submodule.
//! TODO This entire file is a WIP.

#![allow(dead_code)]

use crate::{entities::*, memo::GroupId};
use fxhash::hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalExpression {
    Scan(Scan),
    Filter(Filter),
    Join(Join),
}

/// FIXME: Figure out how to make everything unsigned instead of signed.
impl LogicalExpression {
    pub fn kind(&self) -> i16 {
        match self {
            LogicalExpression::Scan(_) => 0,
            LogicalExpression::Filter(_) => 1,
            LogicalExpression::Join(_) => 2,
        }
    }

    /// Definitions of custom fingerprinting strategies for each kind of logical expression.
    pub fn fingerprint(&self) -> i64 {
        self.fingerprint_with_rewrite(&[])
    }

    /// Calculates the fingerprint of a given expression, but replaces all of the children group IDs
    /// with a new group ID if it is listed in the input `rewrites` list.
    ///
    /// TODO Allow each expression to implement a trait that does this.
    pub fn fingerprint_with_rewrite(&self, rewrites: &[(GroupId, GroupId)]) -> i64 {
        // Closure that rewrites a group ID if needed.
        let rewrite = |x: GroupId| {
            if rewrites.is_empty() {
                return x;
            }

            if let Some(i) = rewrites.iter().position(|(curr, _new)| &x == curr) {
                assert_eq!(rewrites[i].0, x);
                rewrites[i].1
            } else {
                x
            }
        };

        let kind = self.kind() as u16 as usize;
        let hash = match self {
            LogicalExpression::Scan(scan) => hash(scan.table_schema.as_str()),
            LogicalExpression::Filter(filter) => {
                hash(&rewrite(filter.child).0) ^ hash(filter.expression.as_str())
            }
            LogicalExpression::Join(join) => {
                hash(&rewrite(join.left).0)
                    ^ hash(&rewrite(join.right).0)
                    ^ hash(join.expression.as_str())
            }
        };

        // Mask out the bottom 16 bits of `hash` and replace them with `kind`.
        ((hash & !0xFFFF) | kind) as i64
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Scan {
    table_schema: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Filter {
    child: GroupId,
    expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Join {
    left: GroupId,
    right: GroupId,
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

        let kind = value.kind();
        match value {
            LogicalExpression::Scan(scan) => create_logical_expression(
                kind,
                serde_json::to_value(scan).expect("unable to serialize logical `Scan`"),
            ),
            LogicalExpression::Filter(filter) => create_logical_expression(
                kind,
                serde_json::to_value(filter).expect("unable to serialize logical `Filter`"),
            ),
            LogicalExpression::Join(join) => create_logical_expression(
                kind,
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
    use crate::expression::LogicalExpression;

    pub fn scan(table_schema: String) -> LogicalExpression {
        LogicalExpression::Scan(Scan { table_schema })
    }

    pub fn filter(child_group: GroupId, expression: String) -> LogicalExpression {
        LogicalExpression::Filter(Filter {
            child: child_group,
            expression,
        })
    }

    pub fn join(
        left_group: GroupId,
        right_group: GroupId,
        expression: String,
    ) -> LogicalExpression {
        LogicalExpression::Join(Join {
            left: left_group,
            right: right_group,
            expression,
        })
    }
}
