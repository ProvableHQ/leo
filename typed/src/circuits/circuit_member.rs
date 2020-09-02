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

use crate::{Function, Identifier, Type};
use leo_ast::circuits::{
    CircuitFunction as AstCircuitFunction,
    CircuitMember as AstCircuitMember,
    CircuitVariableDefinition as AstCircuitFieldDefinition,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitMember {
    CircuitField(Identifier, Type),
    CircuitFunction(bool, Function),
}

impl<'ast> From<AstCircuitFieldDefinition<'ast>> for CircuitMember {
    fn from(circuit_value: AstCircuitFieldDefinition<'ast>) -> Self {
        CircuitMember::CircuitField(
            Identifier::from(circuit_value.identifier),
            Type::from(circuit_value._type),
        )
    }
}

impl<'ast> From<AstCircuitFunction<'ast>> for CircuitMember {
    fn from(circuit_function: AstCircuitFunction<'ast>) -> Self {
        CircuitMember::CircuitFunction(
            circuit_function._static.is_some(),
            Function::from(circuit_function.function),
        )
    }
}

impl<'ast> From<AstCircuitMember<'ast>> for CircuitMember {
    fn from(object: AstCircuitMember<'ast>) -> Self {
        match object {
            AstCircuitMember::CircuitVariableDefinition(circuit_value) => CircuitMember::from(circuit_value),
            AstCircuitMember::CircuitFunction(circuit_function) => CircuitMember::from(circuit_function),
        }
    }
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitField(ref identifier, ref _type) => write!(f, "{}: {}", identifier, _type),
            CircuitMember::CircuitFunction(ref _static, ref function) => {
                if *_static {
                    write!(f, "static ")?;
                }
                write!(f, "{}", function)
            }
        }
    }
}
