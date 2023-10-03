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

use crate::{Expression, Identifier, Node, NodeID, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A constant declaration statement.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ConstDeclaration {
    /// The place to assign to. As opposed to `DefinitionStatement`, this can only be an identifier
    pub place: Identifier,
    /// The type of the binding, if specified, or inferred otherwise.
    pub type_: Type,
    /// An initializer value for the binding.
    pub value: Expression,
    /// The span excluding the semicolon.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for ConstDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.place)?;
        write!(f, ": {}", self.type_)?;
        write!(f, " = {};", self.value)
    }
}

crate::simple_node_impl!(ConstDeclaration);
