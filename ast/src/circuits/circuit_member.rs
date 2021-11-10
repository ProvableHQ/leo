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
pub enum CircuitMember {
    /// A static constant in a circuit.
    /// For example: `const foobar: u8 = 42;`.
    CircuitConst(
        /// The identifier of the constant.
        Identifier,
        /// The type the constant has.
        Type,
        /// The expression representing the constant's value.
        /// Checked to be of the type above.
        Expression,
    ),
    /// A varible definition in a circuit;
    /// For example: `foobar: u8;`.
    CircuitVariable(
        /// The identifier of the constant.
        Identifier,
        /// The type the constant has.
        Type,
    ),
    /// A function definition in a circuit.
    /// For example: `function bar() -> u8 { return 2u8; }`.
    CircuitFunction(
        /// The function.
        Box<Function>,
    ),
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitConst(ref identifier, ref type_, ref value) => {
                write!(f, "{}: {} = {}", identifier, type_, value)
            }
            CircuitMember::CircuitVariable(ref identifier, ref type_) => write!(f, "{}: {}", identifier, type_),
            CircuitMember::CircuitFunction(ref function) => write!(f, "{}", function),
        }
    }
}
