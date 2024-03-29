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

use leo_span::{Span, Symbol};

use crate::{simple_node_impl, Node, NodeID, ProgramId};
use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hash};

/// A locator that references an external resource.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocatorExpression {
    /// The program that the resource is in.
    pub program: ProgramId,
    /// The name of the resource.
    pub name: Symbol,
    /// A span indicating where the locator occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(LocatorExpression);

impl LocatorExpression {
    /// Constructs a new Locator with `name`, `program` and `id` and a default span.
    pub fn new(program: ProgramId, name: Symbol, id: NodeID) -> Self {
        Self { program, name, span: Span::default(), id }
    }

    /// Check if the Locator name and program matches the other name and program.
    pub fn matches(&self, other: &Self) -> bool {
        self.name == other.name && self.program == other.program
    }
}

impl fmt::Display for LocatorExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.program, self.name)
    }
}
impl fmt::Debug for LocatorExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.program, self.name)
    }
}
