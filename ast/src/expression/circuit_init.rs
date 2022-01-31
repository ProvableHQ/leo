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

use super::*;

/// An initializer for a single field / variable of a circuit initializer expression.
/// That is, in `Foo { bar: 42, baz }`, this is either `bar: 42`, or `baz`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitVariableInitializer {
    /// The name of the field / variable to be initialized.
    pub identifier: Identifier,
    /// The expression to initialize the field with.
    /// When `None`, a binding, in scope, with the name will be used instead.
    pub expression: Option<Expression>,
}

impl fmt::Display for CircuitVariableInitializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(expr) = &self.expression {
            write!(f, "{}: {}", self.identifier, expr)
        } else {
            write!(f, "{}", self.identifier)
        }
    }
}

/// A circuit initialization expression, e.g., `Foo { bar: 42, baz }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitInitExpression {
    /// The name of the structure type to initialize.
    pub name: Identifier,
    /// Initializer expressions for each of the fields in the circuit.
    ///
    /// N.B. Any functions or member constants in the circuit definition
    /// are excluded from this list.
    pub members: Vec<CircuitVariableInitializer>,
    /// A span from `name` to `}`.
    pub span: Span,
}

impl fmt::Display for CircuitInitExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {{", self.name)?;
        for member in self.members.iter() {
            write!(f, "{}", member)?;
            write!(f, ", ")?;
        }
        write!(f, "}}")
    }
}

impl Node for CircuitInitExpression {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
