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

use crate::{Expression, Identifier, Location, Node, NodeID, simple_node_impl};

use leo_span::{Span, Symbol};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hash};

/// A Path in a program.
#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Path {
    /// The program this path belongs to, if set by the user
    program: Option<Identifier>,

    /// The qualifying namespace segments written by the user, excluding the item itself.
    /// e.g., in `foo::bar::baz`, this would be `[foo, bar]`.
    qualifier: Vec<Identifier>,

    /// The final item in the path, e.g., `baz` in `foo::bar::baz`.
    identifier: Identifier,

    /// The target type (i.e. local v.s. global) of this path.
    target: PathTarget,

    /// A span locating where the path occurred in the source.
    pub span: Span,

    /// The ID of the node.
    pub id: NodeID,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PathTarget {
    Unresolved,
    Local(Symbol),
    Global(Location),
}

simple_node_impl!(Path);

impl Path {
    /// Creates a new unresolved `Path` from the given components.
    pub fn new(
        program: Option<Identifier>,
        qualifier: Vec<Identifier>,
        identifier: Identifier,
        span: Span,
        id: NodeID,
    ) -> Self {
        Self { program, qualifier, identifier, target: PathTarget::Unresolved, span, id }
    }

    // ----------------------------
    // Accessors
    // ----------------------------

    /// Returns the final identifier of the path (e.g., `baz` in `foo::bar::baz`).
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns a slice of the qualifier segments (e.g., `[foo, bar]` in `foo::bar::baz`).
    pub fn qualifier(&self) -> &[Identifier] {
        &self.qualifier
    }

    /// Returns an iterator over all segments as `Symbol`s (qualifiers + identifier).
    pub fn segments(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.qualifier.iter().map(|id| id.name).chain(std::iter::once(self.identifier.name))
    }

    /// Returns a `Vec<Symbol>` of the segments.
    pub fn as_symbols(&self) -> Vec<Symbol> {
        self.segments().collect()
    }

    /// Returns the optional program identifier.
    pub fn program(&self) -> Option<&Identifier> {
        self.program.as_ref()
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn id(&self) -> NodeID {
        self.id
    }

    // ----------------------------
    // Resolution helpers
    // ----------------------------

    pub fn is_resolved(&self) -> bool {
        !matches!(self.target, PathTarget::Unresolved)
    }

    pub fn is_local(&self) -> bool {
        matches!(self.target, PathTarget::Local(_))
    }

    pub fn is_global(&self) -> bool {
        matches!(self.target, PathTarget::Global(_))
    }

    pub fn local_symbol(&self) -> Option<Symbol> {
        match self.target {
            PathTarget::Local(sym) => Some(sym),
            _ => None,
        }
    }

    pub fn global_location(&self) -> Option<&Location> {
        match &self.target {
            PathTarget::Global(loc) => Some(loc),
            _ => None,
        }
    }

    /// Returns the `Symbol` if local, panics if not.
    pub fn expect_local(&self) -> Symbol {
        match self.target {
            PathTarget::Local(sym) => sym,
            _ => panic!("Expected a local path, found {:?}", self.target),
        }
    }

    /// Returns the `Location` if global, panics if not.
    pub fn expect_global(&self) -> &Location {
        match &self.target {
            PathTarget::Global(loc) => loc,
            _ => panic!("Expected a global path, found {:?}", self.target),
        }
    }

    pub fn absolute_path(&self) -> Vec<Symbol> {
        match &self.target {
            PathTarget::Local(sym) => vec![*sym],
            PathTarget::Global(loc) => loc.path.clone(),
            PathTarget::Unresolved => panic!("Cannot get absolute path of unresolved path"),
        }
    }

    // ----------------------------
    // Resolution setters (used by resolver)
    // ----------------------------

    /// Resolves this path to a local symbol.
    #[must_use]
    pub fn with_local(self, symbol: Symbol) -> Self {
        debug_assert!(matches!(self.target, PathTarget::Unresolved), "attempted to resolve an already-resolved path");

        Self { target: PathTarget::Local(symbol), ..self }
    }

    /// Resolves this path to a global location.
    #[must_use]
    pub fn with_global(self, location: Location) -> Self {
        debug_assert!(matches!(self.target, PathTarget::Unresolved), "attempted to resolve an already-resolved path");

        Self { target: PathTarget::Global(location), ..self }
    }

    /// Marks this path as unresolved again.
    #[must_use]
    pub fn unresolved(self) -> Self {
        Self { target: PathTarget::Unresolved, ..self }
    }

    /// Returns a new `Path` with the final identifier replaced by `new_symbol`.
    ///
    /// This updates:
    /// - `identifier.name`
    /// - `target`:
    ///   - `Local(_)` → `Local(new_symbol)`
    ///   - `Global(Location)` → same location, but with the final path segment replaced
    ///   - `Unresolved` → unchanged
    #[must_use]
    pub fn with_updated_last_symbol(self, new_symbol: Symbol) -> Self {
        let Path { mut identifier, target, program, qualifier, span, id } = self;

        // Update user-visible identifier
        identifier.name = new_symbol;

        let target = match target {
            PathTarget::Unresolved => PathTarget::Unresolved,

            PathTarget::Local(_) => PathTarget::Local(new_symbol),

            PathTarget::Global(location) => {
                let Location { program, mut path } = location;

                debug_assert!(!path.is_empty(), "global location must have at least one path segment");

                *path.last_mut().unwrap() = new_symbol;

                PathTarget::Global(Location { program, path })
            }
        };

        Self { program, qualifier, identifier, target, span, id }
    }

    /// Resolves an unresolved path into a global path using the current module.
    ///
    /// The resulting global location is:
    /// `[ current_module..., qualifier..., identifier ]`
    ///
    /// This only mutates the `target` field.
    ///
    /// # Debug invariants
    /// - The path must currently be `Unresolved`
    #[must_use]
    pub fn with_module_prefix<I>(self, program: Symbol, current_module: I) -> Self
    where
        I: IntoIterator<Item = Symbol>,
    {
        let Path { program: user_program, qualifier, identifier, target, span, id } = self;

        debug_assert!(
            matches!(target, PathTarget::Unresolved),
            "with_module_prefix may only be called on unresolved paths"
        );

        let mut path: Vec<Symbol> = Vec::new();

        // 1. Current module
        path.extend(current_module);

        // 2. User-written qualifier
        path.extend(qualifier.iter().map(|id| id.name));

        // 3. Final identifier
        path.push(identifier.name);

        let target = PathTarget::Global(Location { program, path });

        Self { program: user_program, qualifier, identifier, target, span, id }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Optional program prefix
        /*if let Some(program) = &self.program {
            write!(f, "{}::", program.name)?;
        }*/

        // Qualifiers
        if !self.qualifier.is_empty() {
            write!(f, "{}::", self.qualifier.iter().map(|id| &id.name).format("::"))?;
        }

        // Final identifier
        write!(f, "{}", self.identifier.name)
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // First, display the user-written path
        write!(f, "{}", self)?;

        // Append resolved info if available
        match &self.target {
            PathTarget::Local(sym) => write!(f, " [local: {}]", sym),
            PathTarget::Global(loc) => {
                write!(f, " [global: {}]", loc.path.iter().map(|s| s.to_string()).format("::"))
            }
            PathTarget::Unresolved => write!(f, " [unresolved]"),
        }
    }
}

impl From<Path> for Expression {
    fn from(value: Path) -> Self {
        Expression::Path(value)
    }
}
