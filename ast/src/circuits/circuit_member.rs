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
    /// Represented as three elements.
    /// const_name - An Identifier representing the constant name.
    /// const_type - The type of the constant.
    /// const_expression - The expression representing the value of the constant.
    CircuitConst(Identifier, Type, Expression),
    /// Represented as two elements.
    /// variable_name - An Identifier representing the variable name.
    /// variable_type - The type of the variable.
    CircuitVariable(Identifier, Type),
    /// Represented as a single element.
    /// function - The function definition.
    CircuitFunction(Box<Function>),
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
