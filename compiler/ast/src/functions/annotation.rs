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

use crate::{Identifier, Node, NodeID, simple_node_impl};

use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An annotation, e.g. @program.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Annotation {
    // TODO: Consider using a symbol instead of an identifier.
    /// The name of the annotation.
    pub identifier: Identifier,
    /// Keys and value associated with the annotation.
    pub map: IndexMap<Symbol, String>,
    /// A span locating where the annotation occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Annotation);

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{}", self.identifier)
    }
}
