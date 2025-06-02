// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{Expression, Identifier, Node, NodeID, Statement, Type};

use leo_span::Span;

use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A `let` or `const` declaration statement.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct DefinitionStatement {
    /// The bindings / variable names to declare.
    pub place: DefinitionPlace,
    /// The types of the bindings, if specified, or inferred otherwise.
    pub type_: Option<Type>,
    /// An initializer value for the bindings.
    pub value: Expression,
    /// The span excluding the semicolon.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum DefinitionPlace {
    Single(Identifier),
    Multiple(Vec<Identifier>),
}

impl fmt::Display for DefinitionPlace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefinitionPlace::Single(id) => id.fmt(f),
            DefinitionPlace::Multiple(ids) => write!(f, "({})", ids.iter().format(", ")),
        }
    }
}

impl fmt::Display for DefinitionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.type_ {
            // For an Err type (as produced by many passes), don't write the type to reduce verbosity.
            Some(Type::Err) | None => write!(f, "let {} = {}", self.place, self.value),
            Some(ty) => write!(f, "let {}: {} = {}", self.place, ty, self.value),
        }
    }
}

impl From<DefinitionStatement> for Statement {
    fn from(value: DefinitionStatement) -> Self {
        Statement::Definition(value)
    }
}

crate::simple_node_impl!(DefinitionStatement);
