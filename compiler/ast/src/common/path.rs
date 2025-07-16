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

use crate::{Expression, Node, NodeID, simple_node_impl};
use snarkvm::{console::program::Identifier as IdentifierCore, prelude::Network};

use leo_span::{Span, Symbol};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

/// A Path in a program.
///
/// Attention - When adding or removing fields from this struct,
/// please remember to update its Serialize and Deserialize implementation
/// to reflect the new struct instantiation.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Path {
    /// The symbol that the user wrote, e.g., `foo`.
    pub path: Vec<Symbol>,
    /// A span locating where the path occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Path);

impl Path {
    /// Constructs a new path with `path` and `id` and a default span.
    pub fn new(path: Vec<Symbol>, id: NodeID) -> Self {
        Self { path, span: Span::default(), id }
    }

    /// Check if the Path path matches the other path.
    pub fn matches(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.iter().format("::"))
    }
}
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.iter().format("::"))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.span == other.span
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.span.hash(state);
    }
}

impl<N: Network> From<&IdentifierCore<N>> for Path {
    fn from(id: &IdentifierCore<N>) -> Self {
        Self { path: vec![Symbol::intern(&id.to_string())], span: Default::default(), id: Default::default() }
    }
}

impl From<Path> for Expression {
    fn from(value: Path) -> Self {
        Expression::Path(value)
    }
}
