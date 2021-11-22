// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{Expression, Function, Identifier, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructMember {
    /// A static constant in a struct.
    /// For example: `const foobar: u8 = 42;`.
    StructConst(
        /// The identifier of the constant.
        Identifier,
        /// The type the constant has.
        Type,
        /// The expression representing the constant's value.
        /// Checked to be of the type above.
        Expression,
    ),
    /// A varible definition in a struct;
    /// For example: `foobar: u8;`.
    StructVariable(
        /// The identifier of the constant.
        Identifier,
        /// The type the constant has.
        Type,
    ),
    /// A function definition in a struct.
    /// For example: `function bar() -> u8 { return 2u8; }`.
    StructFunction(
        /// The function.
        Box<Function>,
    ),
}

impl fmt::Display for StructMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StructMember::StructConst(ref identifier, ref type_, ref value) => {
                write!(f, "{}: {} = {}", identifier, type_, value)
            }
            StructMember::StructVariable(ref identifier, ref type_) => write!(f, "{}: {}", identifier, type_),
            StructMember::StructFunction(ref function) => write!(f, "{}", function),
        }
    }
}
