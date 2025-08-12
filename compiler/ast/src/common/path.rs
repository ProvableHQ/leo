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

use crate::{Expression, Identifier, Node, NodeID, simple_node_impl};
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
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Path {
    /// The path that the user wrote, e.g., `foo::bar`.
    pub segments: Vec<Identifier>,
    /// The fully resolved path. We may not know this until the pass PathResolution pass runs.
    pub absolute_path: Option<Vec<Symbol>>,
    /// A span locating where the path occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Path);

impl Path {
    /// Returns a `Vec<Symbol>` containing the symbolic names of each segment in the path.
    pub fn symbols(&self) -> Vec<Symbol> {
        self.segments.iter().map(|seg| seg.name).collect()
    }

    /// Returns the `Symbol` of the last segment in the path.
    ///
    /// # Panics
    ///
    /// Panics if the path has no segments. Paths are expected to have at least one segment.
    pub fn as_symbol(&self) -> Symbol {
        self.segments.last().expect("a path must always have at least 1 segment").name
    }

    /// Returns the `Identifier` of the last segment in the path.
    ///
    /// # Panics
    ///
    /// Panics if the path has no segments. Paths are expected to have at least one segment.
    pub fn as_identifier(&self) -> Identifier {
        *self.segments.last().expect("a path must always have at least 1 segment")
    }

    /// Returns an optional slice of `Symbol`s representing the resolved absolute path,
    /// or `None` if resolution has not yet occurred.
    pub fn try_absolute_path(&self) -> Option<&[Symbol]> {
        self.absolute_path.as_deref()
    }

    /// Returns a slice of `Symbol`s representing the resolved absolute path.
    ///
    /// # Panics
    ///
    /// Panics if the absolute path has not been resolved yet. This is expected to be
    /// called only after path resolution has occurred.
    pub fn absolute_path(&self) -> &[Symbol] {
        self.absolute_path.as_deref().expect("absolute path must be known at this stage")
    }

    /// Returns a new `Path` instance with the last segment's `Symbol` and the last symbol
    /// in the `absolute_path` (if present) replaced with `new_symbol`.
    ///
    /// Other fields remain unchanged.
    pub fn with_updated_last_symbol(&self, new_symbol: Symbol) -> Self {
        // Clone and update segments
        let mut new_segments = self.segments.clone();
        if let Some(last_segment) = new_segments.last_mut() {
            last_segment.name = new_symbol;
        }

        // Clone and update absolute_path
        let new_absolute_path = self.absolute_path.as_ref().map(|absolute| {
            let mut absolute = absolute.clone();
            if let Some(last_symbol) = absolute.last_mut() {
                *last_symbol = new_symbol;
            }
            absolute
        });

        Path { segments: new_segments, absolute_path: new_absolute_path, span: self.span, id: self.id }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.segments.iter().format("::"))
    }
}
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Display the path segments as well as the absolute_path if available
        write!(f, "{}", self.segments.iter().format("::"))?;
        if let Some(absolute) = &self.absolute_path {
            write!(f, "(::{})", absolute.iter().format("::"))
        } else {
            write!(f, "()")
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments && self.span == other.span
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.segments.hash(state);
        self.span.hash(state);
    }
}

impl<N: Network> From<&IdentifierCore<N>> for Path {
    fn from(id: &IdentifierCore<N>) -> Self {
        let id = Identifier::from(id);
        Self {
            segments: vec![id],
            absolute_path: Some(vec![id.name]),
            span: Default::default(),
            id: Default::default(),
        }
    }
}

impl From<Path> for Expression {
    fn from(value: Path) -> Self {
        Expression::Path(value)
    }
}
