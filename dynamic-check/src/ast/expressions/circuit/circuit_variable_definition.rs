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

use crate::{Expression, ExpressionError, ResolvedNode};
use leo_static_check::{SymbolTable, Type};
use leo_typed::{CircuitVariableDefinition as UnresolvedCircuitVariableDefinition, Identifier};

use serde::{Deserialize, Serialize};

/// A circuit variable with an assigned expression. Used when defining an inline circuit expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitVariableDefinition {
    identifier: Identifier,
    expression: Expression,
}

impl ResolvedNode for CircuitVariableDefinition {
    type Error = ExpressionError;
    type UnresolvedNode = (Option<Type>, UnresolvedCircuitVariableDefinition);

    ///
    /// Type check a circuit variable in an inline circuit expression
    ///
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let expected_type = unresolved.0;
        let circuit_variable = unresolved.1;

        // Resolve circuit variable expression with expected type
        Ok(CircuitVariableDefinition {
            identifier: circuit_variable.identifier,
            expression: Expression::resolve(table, (expected_type, circuit_variable.expression))?,
        })
    }
}
