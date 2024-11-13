use arrow_schema::DataType;
use chrono::NaiveDate;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SerializableOrderedF64(pub OrderedFloat<f64>);

impl Serialize for SerializableOrderedF64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Directly serialize the inner f64 value of the OrderedFloat
        self.0 .0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableOrderedF64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize an f64 and wrap it in an OrderedFloat
        let float = f64::deserialize(deserializer)?;
        Ok(SerializableOrderedF64(OrderedFloat(float)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Value {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float(SerializableOrderedF64),
    String(Arc<str>),
    Bool(bool),
    Date32(i32),
    Decimal128(i128),
    Serialized(Arc<[u8]>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UInt8(x) => write!(f, "{x}(u8)"),
            Self::UInt16(x) => write!(f, "{x}(u16)"),
            Self::UInt32(x) => write!(f, "{x}(u32)"),
            Self::UInt64(x) => write!(f, "{x}(u64)"),
            Self::Int8(x) => write!(f, "{x}(i8)"),
            Self::Int16(x) => write!(f, "{x}(i16)"),
            Self::Int32(x) => write!(f, "{x}(i32)"),
            Self::Int64(x) => write!(f, "{x}(i64)"),
            Self::Int128(x) => write!(f, "{x}(i128)"),
            Self::Float(x) => write!(f, "{}(float)", x.0),
            Self::String(x) => write!(f, "\"{x}\""),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Date32(x) => write!(f, "{x}(date32)"),
            Self::Decimal128(x) => write!(f, "{x}(decimal128)"),
            Self::Serialized(x) => write!(f, "<len:{}>", x.len()),
        }
    }
}

/// The `as_*()` functions do not perform conversions. This is *unlike* the `as`
/// keyword in rust.
///
/// If you want to perform conversions, use the `to_*()` functions.
impl Value {
    pub fn as_u8(&self) -> u8 {
        match self {
            Value::UInt8(i) => *i,
            _ => panic!("Value is not an u8"),
        }
    }

    pub fn as_u16(&self) -> u16 {
        match self {
            Value::UInt16(i) => *i,
            _ => panic!("Value is not an u16"),
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Value::UInt32(i) => *i,
            _ => panic!("Value is not an u32"),
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self {
            Value::UInt64(i) => *i,
            _ => panic!("Value is not an u64"),
        }
    }

    pub fn as_i8(&self) -> i8 {
        match self {
            Value::Int8(i) => *i,
            _ => panic!("Value is not an i8"),
        }
    }

    pub fn as_i16(&self) -> i16 {
        match self {
            Value::Int16(i) => *i,
            _ => panic!("Value is not an i16"),
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            Value::Int32(i) => *i,
            _ => panic!("Value is not an i32"),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Value::Int64(i) => *i,
            _ => panic!("Value is not an i64"),
        }
    }

    pub fn as_i128(&self) -> i128 {
        match self {
            Value::Int128(i) => *i,
            _ => panic!("Value is not an i128"),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Float(i) => *i.0,
            _ => panic!("Value is not an f64"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(i) => *i,
            _ => panic!("Value is not a bool"),
        }
    }

    pub fn as_str(&self) -> Arc<str> {
        match self {
            Value::String(i) => i.clone(),
            _ => panic!("Value is not a string"),
        }
    }

    pub fn as_slice(&self) -> Arc<[u8]> {
        match self {
            Value::Serialized(i) => i.clone(),
            _ => panic!("Value is not a serialized"),
        }
    }

    pub fn convert_to_type(&self, typ: DataType) -> Value {
        match typ {
            DataType::Int32 => Value::Int32(match self {
                Value::Int32(i32) => *i32,
                Value::Int64(i64) => (*i64).try_into().unwrap(),
                _ => panic!("{self} could not be converted into an Int32"),
            }),
            DataType::Int64 => Value::Int64(match self {
                Value::Int64(i64) => *i64,
                Value::Int32(i32) => (*i32).into(),
                _ => panic!("{self} could not be converted into an Int64"),
            }),
            DataType::UInt64 => Value::UInt64(match self {
                Value::Int64(i64) => (*i64).try_into().unwrap(),
                Value::UInt64(i64) => *i64,
                Value::UInt32(i32) => (*i32).into(),
                _ => panic!("{self} could not be converted into an UInt64"),
            }),
            DataType::Date32 => Value::Date32(match self {
                Value::Date32(date32) => *date32,
                Value::String(str) => {
                    let date = NaiveDate::parse_from_str(str, "%Y-%m-%d").unwrap();
                    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                    let duration_since_epoch = date.signed_duration_since(epoch);
                    let days_since_epoch: i32 = duration_since_epoch.num_days() as i32;
                    days_since_epoch
                }
                _ => panic!("{self} could not be converted into an Date32"),
            }),
            _ => unimplemented!("Have not implemented convert_to_type for DataType {typ}"),
        }
    }
}
