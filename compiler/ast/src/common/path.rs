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
    /// The fully resolved path.
    pub resolved_path: Option<Vec<Symbol>>,
    /// A span locating where the path occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Path);

impl Path {
    /// Constructs a new path with `path` and `id` and a default span.
    pub fn new(segments: Vec<Identifier>, id: NodeID) -> Self {
        Self { segments, resolved_path: None, span: Span::default(), id }
    }

    /// Check if the `Path` `self` matches the `Path` `other`.
    pub fn matches(&self, other: &Self) -> bool {
        self.segments == other.segments
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        self.segments.iter().map(|seg| seg.name).collect()
    }

    pub fn as_symbol(&self) -> Symbol {
        self.segments.last().expect("a path must always have at least 1 segment").name
    }

    pub fn as_identifier(&self) -> Identifier {
        *self.segments.last().expect("a path must always have at least 1 segment")
    }

    pub fn try_absolute_path(&self) -> Option<&[Symbol]> {
        self.resolved_path.as_deref()
    }

    pub fn absolute_path(&self) -> &[Symbol] {
        self.resolved_path.as_deref().expect("absolute path must be known at this stage")
    }

    pub fn with_updated_last_symbol(&self, new_symbol: Symbol) -> Self {
        // Clone and update segments
        let mut new_segments = self.segments.clone();
        if let Some(last_segment) = new_segments.last_mut() {
            last_segment.name = new_symbol;
        }

        // Clone and update resolved_path
        let new_resolved_path = self.resolved_path.as_ref().map(|resolved| {
            let mut resolved = resolved.clone();
            if let Some(last_symbol) = resolved.last_mut() {
                *last_symbol = new_symbol;
            }
            resolved
        });

        Path { segments: new_segments, resolved_path: new_resolved_path, span: self.span, id: self.id }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.segments.iter().format("::"))
    }
}
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.segments.iter().format("::"))?;
        if let Some(resolved) = &self.resolved_path {
            write!(f, "(::{})", resolved.iter().format("::"))
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
            resolved_path: Some(vec![id.name]),
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
