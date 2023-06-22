pub use sqlparser::ast::DataType as DataTypeKind;
use sqlparser::ast::Value;

/// Inner data type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataType {
    kind: DataTypeKind,
    nullable: bool,
}

impl DataType {
    #[inline]
    pub const fn new(kind: DataTypeKind, nullable: bool) -> DataType {
        DataType { kind, nullable }
    }
    #[inline]
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
    #[inline]
    pub fn kind(&self) -> DataTypeKind {
        self.kind.clone()
    }
}

pub trait DataTypeExt {
    fn nullable(self) -> DataType;
    fn not_null(self) -> DataType;
}

impl DataTypeExt for DataTypeKind {
    #[inline]
    fn nullable(self) -> DataType {
        DataType::new(self, true)
    }
    #[inline]
    fn not_null(self) -> DataType {
        DataType::new(self, false)
    }
}

const VARCHAR_DEFAULT_LEN: u64 = 256;

pub(crate) type DatabaseIdT = u32;
pub(crate) type SchemaIdT = u32;
pub(crate) type TableIdT = u32;
pub(crate) type ColumnIdT = u32;

/// Primitive SQL value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataValue {
    // NOTE: Null comes first.
    // => NULL is less than any non-NULL values
    Null,
    Bool(bool),
    Int32(i32),
    Float64(f64),
    String(String),
}

impl ToString for DataValue {
    fn to_string(&self) -> String {
        match self {
            Self::Null => String::from("NULL"),
            Self::Bool(v) => v.to_string(),
            Self::Int32(v) => v.to_string(),
            Self::Float64(v) => v.to_string(),
            Self::String(v) => v.to_string(),
        }
    }
}

impl DataValue {
    /// Get the type of value. `None` means NULL.
    pub fn datatype(&self) -> Option<DataType> {
        match self {
            Self::Bool(_) => Some(DataTypeKind::Boolean.not_null()),
            Self::Int32(_) => Some(DataTypeKind::Int(None).not_null()),
            Self::Float64(_) => Some(DataTypeKind::Double.not_null()),
            Self::String(_) => Some(DataTypeKind::Varchar(None).not_null()),
            Self::Null => None,
        }
    }
}

impl From<&Value> for DataValue {
    fn from(v: &Value) -> Self {
        match v {
            // FIXME: float?
            Value::Number(n, _) => {
                if let Ok(int) = n.parse::<i32>() {
                    Self::Int32(int)
                } else if let Ok(float) = n.parse::<f64>() {
                    Self::Float64(float as f64)
                } else {
                    todo!("unsupported number {:?}", n)
                }
            }
            Value::SingleQuotedString(s) => Self::String(s.to_string()),
            Value::DoubleQuotedString(s) => Self::String(s.to_string()),
            Value::Boolean(b) => Self::Bool(*b),
            Value::Null => Self::Null,
            _ => todo!("parse value"),
        }
    }
}
