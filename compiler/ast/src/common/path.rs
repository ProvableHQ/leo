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

use leo_span::{Span, Symbol};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hash};

/// A Path in a program.
#[derive(Clone, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Path {
    /// The qualifying namespace segments written by the user, excluding the item itself.
    /// e.g., in `foo::bar::baz`, this would be `[foo, bar]`.
    qualifier: Vec<Identifier>,

    /// The final item in the path, e.g., `baz` in `foo::bar::baz`.
    identifier: Identifier,

    /// The fully resolved path. We may not know this until the pass PathResolution pass runs.
    /// For path that refer to global items (structs, consts, functions), `absolute_path` is
    /// guaranteed to be set after the pass `PathResolution`.
    absolute_path: Option<Vec<Symbol>>,

    /// A span locating where the path occurred in the source.
    pub span: Span,

    /// The ID of the node.
    pub id: NodeID,
}

simple_node_impl!(Path);

impl Path {
    pub fn new(
        qualifier: Vec<Identifier>,
        identifier: Identifier,
        absolute_path: Option<Vec<Symbol>>,
        span: Span,
        id: NodeID,
    ) -> Self {
        Self { qualifier, identifier, absolute_path, span, id }
    }

    pub fn identifier(&self) -> Identifier {
        self.identifier
    }

    pub fn qualifier(&self) -> &[Identifier] {
        self.qualifier.as_slice()
    }

    /// Returns a `Vec<Symbol>` representing the full symbolic path:
    /// the qualifier segments followed by the final identifier.
    ///
    /// Note: this refers to the user path which is not necessarily the absolute path.
    pub fn as_symbols(&self) -> Vec<Symbol> {
        self.qualifier.iter().map(|segment| segment.name).chain(std::iter::once(self.identifier.name)).collect()
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
    pub fn with_updated_last_symbol(mut self, new_symbol: Symbol) -> Self {
        // Update identifier
        self.identifier.name = new_symbol;

        // Update absolute_path's last symbol if present
        if let Some(ref mut abs_path) = self.absolute_path {
            if let Some(last) = abs_path.last_mut() {
                *last = new_symbol;
            }
        }

        self
    }

    /// Sets `self.absolute_path` to `absolute_path`
    pub fn with_absolute_path(mut self, absolute_path: Option<Vec<Symbol>>) -> Self {
        self.absolute_path = absolute_path;
        self
    }

    /// Sets the `absolute_path` by prepending the given `module_prefix` to the path's
    /// own qualifier and identifier. Returns the updated `Path`.
    pub fn with_module_prefix(mut self, module_prefix: &[Symbol]) -> Self {
        let full_path = module_prefix
            .iter()
            .cloned()
            .chain(self.qualifier.iter().map(|id| id.name))
            .chain(std::iter::once(self.identifier.name))
            .collect();

        self.absolute_path = Some(full_path);
        self
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.qualifier.is_empty() {
            write!(f, "{}", self.identifier)
        } else {
            write!(f, "{}::{}", self.qualifier.iter().format("::"), self.identifier)
        }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print user path (Display impl)
        write!(f, "{self}")?;

        // Print resolved absolute path if available
        if let Some(abs_path) = &self.absolute_path {
            write!(f, "(::{})", abs_path.iter().format("::"))
        } else {
            write!(f, "()")
        }
    }
}

impl From<Path> for Expression {
    fn from(value: Path) -> Self {
        Expression::Path(value)
    }
}
