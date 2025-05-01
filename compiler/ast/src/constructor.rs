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

use crate::{Block, Indent, Node, NodeID};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A constructor definition.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Constructor {
    /// The body of the function.
    pub block: Block,
    /// The entire span of the function definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "async constructor() {{")?;
        for stmt in self.block.statements.iter() {
            writeln!(f, "{}{}", Indent(stmt), stmt.semicolon())?;
        }
        write!(f, "}}")
    }
}

impl fmt::Debug for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq for Constructor {
    fn eq(&self, other: &Self) -> bool {
        self.block == other.block
    }
}

impl Eq for Constructor {}

crate::simple_node_impl!(Constructor);
