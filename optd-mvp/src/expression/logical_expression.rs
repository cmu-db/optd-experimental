//! Definition of logical expressions / relations in our query optimization framework.
//!
//! FIXME: All fields are placeholders.
//!
//! TODO Figure out if each relation should be in a different submodule.
//! TODO This entire file is a WIP.

use crate::{entities::logical_expression::Model, memo::GroupId};
use fxhash::hash;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An interface defining what an in-memory logical expression representation should be able to do.
pub trait LogicalExpression: From<Model> + Into<Model> + Clone + Debug {
    /// Returns the kind of relation / operator node encoded as an integer.
    fn kind(&self) -> i16;

    /// Retrieves the child groups IDs of this logical expression.
    fn children(&self) -> Vec<GroupId>;

    /// Computes the fingerprint of this expression, which should generate an integer for equality
    /// checks that has a low collision rate.
    fn fingerprint(&self) -> i64;

    /// Checks if the current expression is a duplicate of the other expression.
    ///
    /// Note that this is similar to `Eq` and `PartialEq`, but the implementor should be aware that
    /// different expressions can be duplicates of each other without having the exact same data.
    fn is_duplicate(&self, other: &Self) -> bool;

    /// Rewrites the expression to use new child groups IDs, where `rewrites` is a slice of tuples
    /// representing `(old_group_id, new_group_id)`.
    ///
    /// TODO: There's definitely a better way to represent this API
    fn rewrite(&self, rewrites: &[(GroupId, GroupId)]) -> Self;
}

#[derive(Clone, Debug)]
pub enum DefaultLogicalExpression {
    Scan(Scan),
    Filter(Filter),
    Join(Join),
}

impl LogicalExpression for DefaultLogicalExpression {
    fn kind(&self) -> i16 {
        match self {
            Self::Scan(_) => 0,
            Self::Filter(_) => 1,
            Self::Join(_) => 2,
        }
    }

    fn children(&self) -> Vec<GroupId> {
        match self {
            Self::Scan(_) => vec![],
            Self::Filter(filter) => vec![filter.child],
            Self::Join(join) => vec![join.left, join.right],
        }
    }

    fn fingerprint(&self) -> i64 {
        let kind = self.kind() as u16 as usize;
        let hash = match self {
            Self::Scan(scan) => hash(scan.table.as_str()),
            Self::Filter(filter) => hash(&filter.child.0) ^ hash(filter.expression.as_str()),
            Self::Join(join) => {
                // Make sure that there is a difference between `Join(A, B)` and `Join(B, A)`.
                hash(&(join.left.0 + 1))
                    ^ hash(&(join.right.0 + 2))
                    ^ hash(join.expression.as_str())
            }
        };

        // Mask out the bottom 16 bits of `hash` and replace them with `kind`.
        ((hash & !0xFFFF) | kind) as i64
    }

    fn is_duplicate(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Scan(scan_left), Self::Scan(scan_right)) => scan_left.table == scan_right.table,
            (Self::Filter(filter_left), Self::Filter(filter_right)) => {
                filter_left.child == filter_right.child
                    && filter_left.expression == filter_right.expression
            }
            (Self::Join(join_left), Self::Join(join_right)) => {
                join_left.left == join_right.left
                    && join_left.right == join_right.right
                    && join_left.expression == join_right.expression
            }
            _ => false,
        }
    }

    fn rewrite(&self, rewrites: &[(GroupId, GroupId)]) -> Self {
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

        match self {
            Self::Scan(_) => self.clone(),
            Self::Filter(filter) => Self::Filter(Filter {
                child: rewrite(filter.child),
                expression: filter.expression.clone(),
            }),
            Self::Join(join) => Self::Join(Join {
                left: rewrite(join.left),
                right: rewrite(join.right),
                expression: join.expression.clone(),
            }),
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

impl From<Model> for DefaultLogicalExpression {
    fn from(value: Model) -> Self {
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

impl From<DefaultLogicalExpression> for Model {
    fn from(value: DefaultLogicalExpression) -> Model {
        fn create_logical_expression(kind: i16, data: serde_json::Value) -> Model {
            Model {
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

pub use build::*;

mod build {
    use super::*;
    use crate::expression::logical_expression::DefaultLogicalExpression;

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
