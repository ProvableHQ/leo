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

    /// Is this path an absolute path? e.g. `::foo::bar::baz`.
    is_absolute: bool,

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
    /// Creates a new `Path` from the given components.
    ///
    /// - `qualifier`: The namespace segments (e.g., `foo::bar` in `foo::bar::baz`).
    /// - `identifier`: The final item in the path (e.g., `baz`).
    /// - `is_absolute`: Whether the path is absolute (starts with `::`).
    /// - `absolute_path`: Optionally, the fully resolved symbolic path.
    /// - `span`: The source code span for this path.
    /// - `id`: The node ID.
    pub fn new(
        qualifier: Vec<Identifier>,
        identifier: Identifier,
        is_absolute: bool,
        absolute_path: Option<Vec<Symbol>>,
        span: Span,
        id: NodeID,
    ) -> Self {
        Self { qualifier, identifier, is_absolute, absolute_path, span, id }
    }

    /// Returns the final identifier of the path (e.g., `baz` in `foo::bar::baz`).
    pub fn identifier(&self) -> Identifier {
        self.identifier
    }

    /// Returns a slice of the qualifier segments (e.g., `[foo, bar]` in `foo::bar::baz`).
    pub fn qualifier(&self) -> &[Identifier] {
        self.qualifier.as_slice()
    }

    /// Returns `true` if the path is absolute (i.e., starts with `::`).
    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    /// Returns a `Vec<Symbol>` representing the full symbolic path:
    /// the qualifier segments followed by the final identifier.
    ///
    /// Note: this refers to the user path which is not necessarily the absolute path.
    pub fn as_symbols(&self) -> Vec<Symbol> {
        self.qualifier.iter().map(|segment| segment.name).chain(std::iter::once(self.identifier.name)).collect()
    }

    /// Returns an optional vector of `Symbol`s representing the resolved absolute path,
    /// or `None` if resolution has not yet occurred.
    pub fn try_absolute_path(&self) -> Option<Vec<Symbol>> {
        if self.is_absolute { Some(self.as_symbols()) } else { self.absolute_path.clone() }
    }

    /// Returns a vector of `Symbol`s representing the resolved absolute path.
    ///
    /// If the path is not an absolute path, this method panics if the absolute path has not been resolved yet.
    /// For relative paths, this is expected to be called only after path resolution has occurred.
    pub fn absolute_path(&self) -> Vec<Symbol> {
        if self.is_absolute {
            self.as_symbols()
        } else {
            self.absolute_path.as_ref().expect("absolute path must be known at this stage").to_vec()
        }
    }

    /// Converts this `Path` into an absolute path by setting its `is_absolute` flag to `true`.
    ///
    /// This does not alter the qualifier or identifier, nor does it compute or modify
    /// the resolved `absolute_path`.
    pub fn into_absolute(mut self) -> Self {
        self.is_absolute = true;
        self
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
        if self.is_absolute {
            write!(f, "::")?;
        }
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
