//! Definition of logical expressions / relations in our query optimization framework.
//!
//! FIXME: All fields are placeholders.
//!
//! TODO Figure out if each relation should be in a different submodule.
//! TODO This entire file is a WIP.

use crate::{entities::*, memo::GroupId};
use fxhash::hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum DefaultLogicalExpression {
    Scan(Scan),
    Filter(Filter),
    Join(Join),
}

impl DefaultLogicalExpression {
    pub fn kind(&self) -> i16 {
        match self {
            DefaultLogicalExpression::Scan(_) => 0,
            DefaultLogicalExpression::Filter(_) => 1,
            DefaultLogicalExpression::Join(_) => 2,
        }
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
            DefaultLogicalExpression::Scan(scan) => hash(scan.table.as_str()),
            DefaultLogicalExpression::Filter(filter) => {
                hash(&rewrite(filter.child).0) ^ hash(filter.expression.as_str())
            }
            DefaultLogicalExpression::Join(join) => {
                // Make sure that there is a difference between `Join(A, B)` and `Join(B, A)`.
                hash(&(rewrite(join.left).0 + 1))
                    ^ hash(&(rewrite(join.right).0 + 2))
                    ^ hash(join.expression.as_str())
            }
        };

        // Mask out the bottom 16 bits of `hash` and replace them with `kind`.
        ((hash & !0xFFFF) | kind) as i64
    }

    /// Checks equality between two expressions, with both expression rewriting their child group
    /// IDs according to the input `rewrites` list.
    pub fn eq_with_rewrite(&self, other: &Self, rewrites: &[(GroupId, GroupId)]) -> bool {
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

        match (self, other) {
            (
                DefaultLogicalExpression::Scan(scan_left),
                DefaultLogicalExpression::Scan(scan_right),
            ) => scan_left.table == scan_right.table,
            (
                DefaultLogicalExpression::Filter(filter_left),
                DefaultLogicalExpression::Filter(filter_right),
            ) => {
                rewrite(filter_left.child) == rewrite(filter_right.child)
                    && filter_left.expression == filter_right.expression
            }
            (
                DefaultLogicalExpression::Join(join_left),
                DefaultLogicalExpression::Join(join_right),
            ) => {
                rewrite(join_left.left) == rewrite(join_right.left)
                    && rewrite(join_left.right) == rewrite(join_right.right)
                    && join_left.expression == join_right.expression
            }
            _ => false,
        }
    }

    pub fn children(&self) -> Vec<GroupId> {
        match self {
            DefaultLogicalExpression::Scan(_) => vec![],
            DefaultLogicalExpression::Filter(filter) => vec![filter.child],
            DefaultLogicalExpression::Join(join) => vec![join.left, join.right],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scan {
    table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filter {
    child: GroupId,
    expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Join {
    left: GroupId,
    right: GroupId,
    expression: String,
}

/// TODO Use a macro.
impl From<logical_expression::Model> for DefaultLogicalExpression {
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

/// TODO Use a macro.
impl From<DefaultLogicalExpression> for logical_expression::Model {
    fn from(value: DefaultLogicalExpression) -> logical_expression::Model {
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
            DefaultLogicalExpression::Scan(scan) => create_logical_expression(
                kind,
                serde_json::to_value(scan).expect("unable to serialize logical `Scan`"),
            ),
            DefaultLogicalExpression::Filter(filter) => create_logical_expression(
                kind,
                serde_json::to_value(filter).expect("unable to serialize logical `Filter`"),
            ),
            DefaultLogicalExpression::Join(join) => create_logical_expression(
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
    use crate::expression::DefaultLogicalExpression;

    pub fn scan(table_schema: String) -> DefaultLogicalExpression {
        DefaultLogicalExpression::Scan(Scan {
            table: table_schema,
        })
    }

    pub fn filter(child_group: GroupId, expression: String) -> DefaultLogicalExpression {
        DefaultLogicalExpression::Filter(Filter {
            child: child_group,
            expression,
        })
    }

    pub fn join(
        left_group: GroupId,
        right_group: GroupId,
        expression: String,
    ) -> DefaultLogicalExpression {
        DefaultLogicalExpression::Join(Join {
            left: left_group,
            right: right_group,
            expression,
        })
    }
}
