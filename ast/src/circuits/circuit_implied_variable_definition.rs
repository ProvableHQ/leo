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

use crate::Expression;
use crate::Identifier;
use leo_grammar::circuits::CircuitImpliedVariable;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitImpliedVariableDefinition {
    pub identifier: Identifier,
    pub expression: Expression,
}

impl<'ast> From<CircuitImpliedVariable<'ast>> for CircuitImpliedVariableDefinition {
    fn from(member: CircuitImpliedVariable<'ast>) -> Self {
        match member {
            CircuitImpliedVariable::CircuitVariable(circuit_variable) => Self {
                identifier: Identifier::from(circuit_variable.identifier),
                expression: Expression::from(circuit_variable.expression),
            },
            CircuitImpliedVariable::Identifier(identifier) => Self {
                identifier: Identifier::from(identifier.clone()),
                expression: Expression::from(identifier),
            },
        }
    }
}
