use std::sync::Arc;

use arrow_schema::{DataType, IntervalUnit};
use optd_persistent::cost_model::interface::AttrType;
use serde::{Deserialize, Serialize};

use crate::common::{
    nodes::{ArcPredicateNode, PredicateNode, PredicateType, ReprPredicateNode},
    values::{SerializableOrderedF64, Value},
};

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

impl ConstantType {
    pub fn get_data_type_from_value(value: &Value) -> Self {
        match value {
            Value::Bool(_) => ConstantType::Bool,
            Value::String(_) => ConstantType::Utf8String,
            Value::UInt8(_) => ConstantType::UInt8,
            Value::UInt16(_) => ConstantType::UInt16,
            Value::UInt32(_) => ConstantType::UInt32,
            Value::UInt64(_) => ConstantType::UInt64,
            Value::Int8(_) => ConstantType::Int8,
            Value::Int16(_) => ConstantType::Int16,
            Value::Int32(_) => ConstantType::Int32,
            Value::Int64(_) => ConstantType::Int64,
            Value::Float(_) => ConstantType::Float64,
            Value::Date32(_) => ConstantType::Date,
            _ => unimplemented!("get_data_type_from_value() not implemented for value {value}"),
        }
    }

    // TODO: current DataType and ConstantType are not 1 to 1 mapping
    // optd schema stores constantType from data type in catalog.get
    // for decimal128, the precision is lost
    pub fn from_data_type(data_type: DataType) -> Self {
        match data_type {
            DataType::Binary => ConstantType::Binary,
            DataType::Boolean => ConstantType::Bool,
            DataType::UInt8 => ConstantType::UInt8,
            DataType::UInt16 => ConstantType::UInt16,
            DataType::UInt32 => ConstantType::UInt32,
            DataType::UInt64 => ConstantType::UInt64,
            DataType::Int8 => ConstantType::Int8,
            DataType::Int16 => ConstantType::Int16,
            DataType::Int32 => ConstantType::Int32,
            DataType::Int64 => ConstantType::Int64,
            DataType::Float64 => ConstantType::Float64,
            DataType::Date32 => ConstantType::Date,
            DataType::Interval(IntervalUnit::MonthDayNano) => ConstantType::IntervalMonthDateNano,
            DataType::Utf8 => ConstantType::Utf8String,
            DataType::Decimal128(_, _) => ConstantType::Decimal,
            _ => unimplemented!("no conversion to ConstantType for DataType {data_type}"),
        }
    }

    pub fn into_data_type(&self) -> DataType {
        match self {
            ConstantType::Binary => DataType::Binary,
            ConstantType::Bool => DataType::Boolean,
            ConstantType::UInt8 => DataType::UInt8,
            ConstantType::UInt16 => DataType::UInt16,
            ConstantType::UInt32 => DataType::UInt32,
            ConstantType::UInt64 => DataType::UInt64,
            ConstantType::Int8 => DataType::Int8,
            ConstantType::Int16 => DataType::Int16,
            ConstantType::Int32 => DataType::Int32,
            ConstantType::Int64 => DataType::Int64,
            ConstantType::Float64 => DataType::Float64,
            ConstantType::Date => DataType::Date32,
            ConstantType::IntervalMonthDateNano => DataType::Interval(IntervalUnit::MonthDayNano),
            ConstantType::Decimal => DataType::Float64,
            ConstantType::Utf8String => DataType::Utf8,
        }
    }

    pub fn from_persistent_attr_type(attr_type: AttrType) -> Self {
        match attr_type {
            AttrType::Integer => ConstantType::Int32,
            AttrType::Float => ConstantType::Float64,
            AttrType::Varchar => ConstantType::Utf8String,
            AttrType::Boolean => ConstantType::Bool,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConstantPred(pub ArcPredicateNode);

impl ConstantPred {
    pub fn new(value: Value) -> Self {
        let typ = ConstantType::get_data_type_from_value(&value);
        Self::new_with_type(value, typ)
    }

    pub fn new_with_type(value: Value, typ: ConstantType) -> Self {
        ConstantPred(
            PredicateNode {
                typ: PredicateType::Constant(typ),
                children: vec![],
                data: Some(value),
            }
            .into(),
        )
    }

    pub fn bool(value: bool) -> Self {
        Self::new_with_type(Value::Bool(value), ConstantType::Bool)
    }

    pub fn string(value: impl AsRef<str>) -> Self {
        Self::new_with_type(
            Value::String(value.as_ref().into()),
            ConstantType::Utf8String,
        )
    }

    pub fn uint8(value: u8) -> Self {
        Self::new_with_type(Value::UInt8(value), ConstantType::UInt8)
    }

    pub fn uint16(value: u16) -> Self {
        Self::new_with_type(Value::UInt16(value), ConstantType::UInt16)
    }

    pub fn uint32(value: u32) -> Self {
        Self::new_with_type(Value::UInt32(value), ConstantType::UInt32)
    }

    pub fn uint64(value: u64) -> Self {
        Self::new_with_type(Value::UInt64(value), ConstantType::UInt64)
    }

    pub fn int8(value: i8) -> Self {
        Self::new_with_type(Value::Int8(value), ConstantType::Int8)
    }

    pub fn int16(value: i16) -> Self {
        Self::new_with_type(Value::Int16(value), ConstantType::Int16)
    }

    pub fn int32(value: i32) -> Self {
        Self::new_with_type(Value::Int32(value), ConstantType::Int32)
    }

    pub fn int64(value: i64) -> Self {
        Self::new_with_type(Value::Int64(value), ConstantType::Int64)
    }

    pub fn interval_month_day_nano(value: i128) -> Self {
        Self::new_with_type(Value::Int128(value), ConstantType::IntervalMonthDateNano)
    }

    pub fn float64(value: f64) -> Self {
        Self::new_with_type(
            Value::Float(SerializableOrderedF64(value.into())),
            ConstantType::Float64,
        )
    }

    pub fn date(value: i64) -> Self {
        Self::new_with_type(Value::Int64(value), ConstantType::Date)
    }

    pub fn decimal(value: f64) -> Self {
        Self::new_with_type(
            Value::Float(SerializableOrderedF64(value.into())),
            ConstantType::Decimal,
        )
    }

    pub fn serialized(value: Arc<[u8]>) -> Self {
        Self::new_with_type(Value::Serialized(value), ConstantType::Binary)
    }

    /// Gets the constant value.
    pub fn value(&self) -> Value {
        self.0.data.clone().unwrap()
    }

    pub fn constant_type(&self) -> ConstantType {
        if let PredicateType::Constant(typ) = self.0.typ {
            typ
        } else {
            panic!("not a constant")
        }
    }
}

impl ReprPredicateNode for ConstantPred {
    fn into_pred_node(self) -> ArcPredicateNode {
        self.0
    }

    fn from_pred_node(rel_node: ArcPredicateNode) -> Option<Self> {
        if let PredicateType::Constant(_) = rel_node.typ {
            Some(Self(rel_node))
        } else {
            None
        }
    }
}
