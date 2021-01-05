// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_gadgets::signed_integer::*;

use snarkvm_models::gadgets::utilities::{boolean::Boolean, uint::*};
use std::fmt;

/// An intermediate value format that can be converted into a `ConstrainedValue` for the compiler
/// TODO(collinc97): implement other constrained values
#[derive(Clone)]
pub enum Value {
    Boolean(Boolean),

    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),

    I8(Int8),
    I16(Int16),
    I32(Int32),
    I64(Int64),
    I128(Int128),

    Array(Vec<Value>),
    Tuple(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_option = match self {
            Value::Boolean(value) => value.get_value().map(|v| v.to_string()),
            Value::U8(value) => value.value.map(|v| v.to_string()),
            Value::U16(value) => value.value.map(|v| v.to_string()),
            Value::U32(value) => value.value.map(|v| v.to_string()),
            Value::U64(value) => value.value.map(|v| v.to_string()),
            Value::U128(value) => value.value.map(|v| v.to_string()),
            Value::I8(value) => value.value.map(|v| v.to_string()),
            Value::I16(value) => value.value.map(|v| v.to_string()),
            Value::I32(value) => value.value.map(|v| v.to_string()),
            Value::I64(value) => value.value.map(|v| v.to_string()),
            Value::I128(value) => value.value.map(|v| v.to_string()),
            Value::Array(values) => {
                let string = values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "[{}]", string)?;

                Some("".to_owned())
            }
            Value::Tuple(values) => {
                let string = values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "[{}]", string)?;

                Some("".to_owned())
            }
        };

        let string = string_option.unwrap_or_else(|| "[input]".to_owned());

        write!(f, "{}", string)
    }
}
