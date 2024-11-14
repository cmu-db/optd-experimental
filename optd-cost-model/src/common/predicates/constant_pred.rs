use serde::{Deserialize, Serialize};

/// TODO: documentation
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ConstantType {
    Bool,
    Utf8String,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float64,
    Date,
    IntervalMonthDateNano,
    Decimal,
    Binary,
}
