//! This module contains items related to the memo table, which is key to the Cascades query
//! optimization framework.
//!
//! TODO more docs.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A new type of an integer identifying a unique group.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct GroupId(pub i32);

/// A new type of an integer identifying a unique logical expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalExpressionId(pub i32);

/// A new type of an integer identifying a unique physical expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalExpressionId(pub i32);

/// A status enum representing the different states a group can be during query optimization.
#[repr(u8)]
pub enum GroupStatus {
    InProgress = 0,
    Explored = 1,
    Optimized = 2,
}

/// The different kinds of errors that might occur while running operations on a memo table.
#[derive(Error, Debug)]
pub enum MemoError {
    #[error("unknown group ID {0:?}")]
    UnknownGroup(GroupId),
    #[error("unknown logical expression ID {0:?}")]
    UnknownLogicalExpression(LogicalExpressionId),
    #[error("unknown physical expression ID {0:?}")]
    UnknownPhysicalExpression(PhysicalExpressionId),
    #[error("invalid expression encountered")]
    InvalidExpression,
}

mod persistent;
