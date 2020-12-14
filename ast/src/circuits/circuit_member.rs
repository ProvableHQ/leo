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
use leo_grammar::{
    circuits::{CircuitMember as GrammarCircuitMember, CircuitVariableDefinition as GrammarCircuitVariableDefinition},
    functions::Function as GrammarFunction,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitMember {
    // (variable_name, variable_type)
    CircuitVariable(Identifier, Type),
    // (function)
    CircuitFunction(Function),
}

impl<'ast> From<GrammarCircuitVariableDefinition<'ast>> for CircuitMember {
    fn from(circuit_value: GrammarCircuitVariableDefinition<'ast>) -> Self {
        CircuitMember::CircuitVariable(
            Identifier::from(circuit_value.identifier),
            Type::from(circuit_value.type_),
        )
    }
}

impl<'ast> From<GrammarFunction<'ast>> for CircuitMember {
    fn from(circuit_function: GrammarFunction<'ast>) -> Self {
        CircuitMember::CircuitFunction(Function::from(circuit_function))
    }
}

impl<'ast> From<GrammarCircuitMember<'ast>> for CircuitMember {
    fn from(object: GrammarCircuitMember<'ast>) -> Self {
        match object {
            GrammarCircuitMember::CircuitVariableDefinition(circuit_value) => CircuitMember::from(circuit_value),
            GrammarCircuitMember::CircuitFunction(circuit_function) => CircuitMember::from(circuit_function),
        }
    }
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitVariable(ref identifier, ref type_) => write!(f, "{}: {}", identifier, type_),
            CircuitMember::CircuitFunction(ref function) => write!(f, "{}", function),
        }
    }
}
