// Copyright (C) 2019-2023 Aleo Systems Inc.
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
use leo_span::sym;

/// An initializer for a single field / variable of a struct initializer expression.
/// That is, in `Foo { bar: 42, baz }`, this is either `bar: 42`, or `baz`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructVariableInitializer {
    /// The name of the field / variable to be initialized.
    pub identifier: Identifier,
    /// The expression to initialize the field with.
    /// When `None`, a binding, in scope, with the name will be used instead.
    pub expression: Option<Expression>,
    /// The span of the node.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

crate::simple_node_impl!(StructVariableInitializer);

impl fmt::Display for StructVariableInitializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(expr) = &self.expression {
            write!(f, "{}: {expr}", self.identifier)
        } else {
            write!(f, "{}", self.identifier)
        }
    }
}

/// A struct initialization expression, e.g., `Foo { bar: 42, baz }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructExpression {
    /// The name of the structure type to initialize.
    pub name: Identifier,
    /// Initializer expressions for each of the fields in the struct.
    ///
    /// N.B. Any functions or member constants in the struct definition
    /// are excluded from this list.
    pub members: Vec<StructVariableInitializer>,
    /// A span from `name` to `}`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl StructExpression {
    /// Returns true if the record has all required fields and visibility.
    pub fn check_record(&self) -> bool {
        let has_member = |symbol| self.members.iter().any(|variable| variable.identifier.name == symbol);

        has_member(sym::owner) && has_member(sym::_nonce)
    }

    /// Returns the struct as a record interface with visibility.
    pub fn to_record_string(&self) -> String {
        format!(
            "{{{}}}",
            self.members
                .iter()
                .map(|variable| {
                    // Write default visibility.
                    if variable.identifier.name == sym::_nonce {
                        format!("{variable}.public")
                    } else {
                        format!("{variable}.private")
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl fmt::Display for StructExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.members.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

crate::simple_node_impl!(StructExpression);
