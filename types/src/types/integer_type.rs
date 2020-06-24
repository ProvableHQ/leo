use leo_ast::types::IntegerType as AstIntegerType;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Explicit integer type
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl From<AstIntegerType> for IntegerType {
    fn from(integer_type: AstIntegerType) -> Self {
        match integer_type {
            AstIntegerType::U8Type(_type) => IntegerType::U8,
            AstIntegerType::U16Type(_type) => IntegerType::U16,
            AstIntegerType::U32Type(_type) => IntegerType::U32,
            AstIntegerType::U64Type(_type) => IntegerType::U64,
            AstIntegerType::U128Type(_type) => IntegerType::U128,
        }
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntegerType::U8 => write!(f, "u8"),
            IntegerType::U16 => write!(f, "u16"),
            IntegerType::U32 => write!(f, "u32"),
            IntegerType::U64 => write!(f, "u64"),
            IntegerType::U128 => write!(f, "u128"),
        }
    }
}
