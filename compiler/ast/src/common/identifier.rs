// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{Expression, Node, NodeID, Path, simple_node_impl};

use leo_span::{Span, Symbol};

use snarkvm::{console::program::Identifier as IdentifierCore, prelude::Network};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

/// An identifier in a program.
///
/// Attention - When adding or removing fields from this struct,
/// please remember to update its Serialize and Deserialize implementation
/// to reflect the new struct instantiation.
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Identifier {
    /// The symbol that the user wrote, e.g., `foo`.
    pub name: Symbol,
    /// A span locating where the identifier occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Identifier);

impl Identifier {
    /// Constructs a new identifier with `name` and `id` and a default span.
    pub fn new(name: Symbol, id: NodeID) -> Self {
        Self { name, span: Span::default(), id }
    }

    /// Check if the Identifier name matches the other name.
    pub fn matches(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}
impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.span == other.span
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.span.hash(state);
    }
}

impl<N: Network> From<&IdentifierCore<N>> for Identifier {
    fn from(id: &IdentifierCore<N>) -> Self {
        Self { name: Symbol::intern(&id.to_string()), span: Default::default(), id: Default::default() }
    }
}

// Converts an `Identifier` to an unresolved `Path` expression
// It's up to the caller of this method to figure out how to resolve the resulting `Path`.
impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Expression::Path(crate::Path::from(value))
    }
}

// Converts an `Identifier` to a `Path`.
// It's up to the caller of this method to figure out how to resolve the resulting `Path`.
impl From<Identifier> for Path {
    fn from(value: Identifier) -> Self {
        Path::new(None, vec![], value, value.span, value.id)
    }
}
