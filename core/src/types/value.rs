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
use snarkos_models::gadgets::utilities::{boolean::Boolean, uint::*};

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
