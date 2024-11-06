use crate::entities::*;
use std::hash::{DefaultHasher, Hash, Hasher};

/// All of the different types of fixed logical operators.
///
/// Note that there could be more operators that the memo table must support that are not enumerated
/// in this enum, as there can be up to `2^16` different types of operators.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i16)]
pub enum LogicalOperator {
    Scan,
    Join,
}

/// All of the different types of fixed physical operators.
///
/// Note that there could be more operators that the memo table must support that are not enumerated
/// in this enum, as there can be up to `2^16` different types of operators.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i16)]
pub enum PhysicalOperator {
    TableScan,
    IndexScan,
    NestedLoopJoin,
    HashJoin,
}

/// A method to generate a fingerprint used to efficiently check if two
/// expressions are equivalent.
///
/// TODO actually make efficient.
fn fingerprint(variant_tag: i16, data: &serde_json::Value) -> i64 {
    let mut hasher = DefaultHasher::new();

    variant_tag.hash(&mut hasher);
    data.hash(&mut hasher);

    hasher.finish() as i64
}

impl logical_expression::Model {
    /// Creates a new logical expression with an unset `id` and `group_id`.
    pub fn new(variant_tag: LogicalOperator, data: serde_json::Value) -> Self {
        let tag = variant_tag as i16;
        let fingerprint = fingerprint(tag, &data);

        Self {
            id: 0,
            group_id: 0,
            fingerprint,
            variant_tag: tag,
            data,
        }
    }
}

impl physical_expression::Model {
    /// Creates a new physical expression with an unset `id` and `group_id`.
    pub fn new(variant_tag: PhysicalOperator, data: serde_json::Value) -> Self {
        let tag = variant_tag as i16;
        let fingerprint = fingerprint(tag, &data);

        Self {
            id: 0,
            group_id: 0,
            fingerprint,
            variant_tag: tag,
            data,
        }
    }
}
