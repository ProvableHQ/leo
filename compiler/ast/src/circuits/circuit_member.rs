// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{Identifier, Type};
use leo_span::Symbol;

use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(clippy::large_enum_variant)]
/// A member of a circuit definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitMember {
    // CAUTION: circuit constants are unstable for Leo testnet3.
    // /// A static constant in a circuit.
    // /// For example: `const foobar: u8 = 42;`.
    // CircuitConst(
    //     /// The identifier of the constant.
    //     Identifier,
    //     /// The type the constant has.
    //     Type,
    //     /// The expression representing the constant's value.
    //     /// Checked to be of the type above.
    //     Expression,
    // ),
    /// A variable definition in a circuit;
    /// For example: `foobar: u8;`.
    CircuitVariable(
        /// The identifier of the constant.
        Identifier,
        /// The type the constant has.
        Type,
    ),
    // CAUTION: circuit functions are unstable for Leo testnet3.
    // /// A function definition in a circuit.
    // /// For example: `function bar() -> u8 { return 2u8; }`.
    // CircuitFunction(
    //     /// The function.
    //     Box<Function>,
    // ),
}

impl CircuitMember {
    /// Returns the name of the circuit member without span.
    pub fn name(&self) -> Symbol {
        match self {
            CircuitMember::CircuitVariable(ident, _type) => ident.name,
        }
    }
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitVariable(ref identifier, ref type_) => write!(f, "{}: {}", identifier, type_),
        }
    }
}
