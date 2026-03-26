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

use crate::{Expression, Identifier, Location, Node, NodeID, ProgramId, simple_node_impl};

use leo_span::{Span, Symbol, with_session_globals};

use indexmap::IndexSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hash};

/// A Path in a program.
#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Path {
    /// The program this path belongs to, if set by the user
    user_program: Option<ProgramId>,

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
    ///
    /// - `user_program`: An optional program name (e.g. `credits` in `credits.aleo::Bar`)
    /// - `qualifier`: The namespace segments (e.g., `foo::bar` in `foo::bar::baz`).
    /// - `identifier`: The final item in the path (e.g., `baz`).
    /// - `span`: The source code span for this path.
    /// - `id`: The node ID.
    pub fn new(
        user_program: Option<ProgramId>,
        qualifier: Vec<Identifier>,
        identifier: Identifier,
        span: Span,
        id: NodeID,
    ) -> Self {
        Self { user_program, qualifier, identifier, target: PathTarget::Unresolved, span, id }
    }

    /// Returns the final identifier of the path (e.g., `baz` in `foo::bar::baz`).
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns a slice of the qualifier segments (e.g., `[foo, bar]` in `foo::bar::baz`).
    pub fn qualifier(&self) -> &[Identifier] {
        &self.qualifier
    }

    /// Returns an iterator over all segments as `Symbol`s (qualifiers + identifier).
    pub fn segments_iter(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.qualifier.iter().map(|id| id.name).chain(std::iter::once(self.identifier.name))
    }

    /// Returns a `Vec<Symbol>` of the segments.
    pub fn segments(&self) -> Vec<Symbol> {
        self.segments_iter().collect()
    }

    /// Returns the optional program identifier.
    pub fn user_program(&self) -> Option<&ProgramId> {
        self.user_program.as_ref()
    }

    /// Returns `self` after setting it `user_program` field to `user_program`.
    pub fn with_user_program(mut self, user_program: ProgramId) -> Self {
        self.user_program = Some(user_program);
        self
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn id(&self) -> NodeID {
        self.id
    }

    pub fn is_resolved(&self) -> bool {
        !matches!(self.target, PathTarget::Unresolved)
    }

    pub fn is_local(&self) -> bool {
        matches!(self.target, PathTarget::Local(_))
    }

    pub fn is_global(&self) -> bool {
        matches!(self.target, PathTarget::Global(_))
    }

    /// Returns the program symbol this path refers to, if known.
    ///
    /// Priority:
    /// 1. User-written program qualifier (e.g. `foo.aleo::bar::baz`)
    /// 2. Resolved global target program
    /// 3. None (unresolved or local)
    pub fn program(&self) -> Option<Symbol> {
        if let Some(id) = &self.user_program {
            return Some(id.as_symbol());
        }

        match &self.target {
            PathTarget::Global(location) => Some(location.program),
            _ => None,
        }
    }

    /// Returns the `Symbol` if local, `None` if not.
    pub fn try_local_symbol(&self) -> Option<Symbol> {
        match self.target {
            PathTarget::Local(sym) => Some(sym),
            _ => None,
        }
    }

    /// Returns the `Location` if global, `None` if not.
    pub fn try_global_location(&self) -> Option<&Location> {
        match &self.target {
            PathTarget::Global(loc) => Some(loc),
            _ => None,
        }
    }

    /// Returns the `Symbol` if local, panics if not.
    pub fn expect_local_symbol(&self) -> Symbol {
        match self.target {
            PathTarget::Local(sym) => sym,
            _ => panic!("Expected a local path, found {:?}", self.target),
        }
    }

    /// Returns the `Location` if global, panics if not.
    pub fn expect_global_location(&self) -> &Location {
        match &self.target {
            PathTarget::Global(loc) => loc,
            _ => panic!("Expected a global path, found {:?}", self.target),
        }
    }

    /// Resolves this path to a local symbol.
    pub fn to_local(self) -> Self {
        Self { target: PathTarget::Local(self.identifier.name), ..self }
    }

    /// Resolves this path to a global location.
    pub fn to_global(self, location: Location) -> Self {
        Self { target: PathTarget::Global(location), ..self }
    }

    /// Returns a new `Path` with the final identifier replaced by `new_symbol`.
    ///
    /// This updates:
    /// - `identifier.name`
    /// - `target`:
    ///   - `Local(_)` → `Local(new_symbol)`
    ///   - `Global(Location)` → same location, but with the final path segment replaced
    ///   - `Unresolved` → unchanged
    pub fn with_updated_last_symbol(self, new_symbol: Symbol) -> Self {
        let Path { mut identifier, target, user_program, qualifier, span, id } = self;

        // Update user-visible identifier
        identifier.name = new_symbol;

        let target = match target {
            PathTarget::Unresolved => PathTarget::Unresolved,

            PathTarget::Local(_) => PathTarget::Local(new_symbol),

            PathTarget::Global(location) => {
                let Location { program, mut path } = location;

                assert!(!path.is_empty(), "global location must have at least one path segment");

                *path.last_mut().unwrap() = new_symbol;

                PathTarget::Global(Location { program, path })
            }
        };

        Self { user_program, qualifier, identifier, target, span, id }
    }

    /// Resolves this path as a global path within the current module context.
    ///
    /// This function converts a user-written path into a fully qualified
    /// [`PathTarget::Global`] by determining which program the path belongs to
    /// and constructing the corresponding module path.
    ///
    /// Resolution follows two main cases:
    ///
    /// 1. **External library access**
    ///    If the path does not explicitly specify a program (`user_program` is `None`)
    ///    and the first qualifier segment matches a known external library name,
    ///    that segment is interpreted as the target program. The remaining qualifier
    ///    segments and identifier form the path inside that program.
    ///
    /// 2. **Local or explicitly-qualified program access**
    ///    Otherwise, the path is resolved relative to the current module context.
    ///    The final location is constructed by combining:
    ///      - the current module path,
    ///      - any user-written qualifier segments, and
    ///      - the final identifier.
    ///
    /// If the user explicitly wrote a program (via `user_program`), it overrides
    /// the default `program` parameter. Otherwise, the current program is used.
    ///
    /// Importantly, this transformation **does not modify the user-written syntax**
    /// (`user_program`, `qualifier`, `identifier`). It only determines the internal
    /// `target` used during later compilation stages.
    pub fn resolve_as_global_in_module<I>(
        self,
        program: Symbol,
        external_libs: &IndexSet<Symbol>,
        current_module: I,
    ) -> Self
    where
        I: IntoIterator<Item = Symbol>,
    {
        let Path { user_program, qualifier, identifier, span, id, .. } = self;

        // Case 1: The path starts with a known external library name and the user
        // did not explicitly specify a program. In this situation we interpret
        // the first qualifier segment as the program name.
        if let Some(first) = qualifier.first()
            && user_program.is_none()
            && external_libs.contains(&first.name)
        {
            // Build the path within the external library by skipping the
            // first qualifier (the library name itself).
            let mut path: Vec<Symbol> = qualifier.iter().skip(1).map(|id| id.name).collect();
            path.push(identifier.name);

            let target = PathTarget::Global(Location { program: first.name, path });

            Self { user_program: None, qualifier, identifier, target, span, id }
        } else {
            // Case 2: Resolve relative to the current module.
            //
            // Construct the path by concatenating:
            //   current_module + user qualifier + identifier.
            let mut path: Vec<Symbol> = Vec::new();
            path.extend(current_module);
            path.extend(qualifier.iter().map(|id| id.name));
            path.push(identifier.name);

            // Determine which program this location belongs to:
            //   - use the explicitly written program if provided
            //   - otherwise fall back to the current program.
            let target = PathTarget::Global(Location {
                program: user_program.map(|id| id.as_symbol()).unwrap_or(program),
                path,
            });

            Self { user_program, qualifier, identifier, target, span, id }
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determine the program prefix and separator:
        //
        // 1. If the user explicitly wrote a program (e.g. `credits.aleo::Foo`), always use it
        //    with a `::` separator.
        //
        // 2. Otherwise fall back to the resolved global location's program, but only when it is
        //    an `.aleo` program.  `.aleo` programs never appear in the qualifier, so we must
        //    reconstruct the prefix here to produce readable error messages like
        //    `parent.aleo::Foo` vs `child.aleo::Foo`.
        //
        // 3. Library programs (no `.aleo` suffix) already have their name as the first qualifier
        //    segment (e.g. `math_lib::Foo`), so adding a prefix here would double-print it.
        if let Some(pid) = &self.user_program {
            write!(f, "{}::", pid.as_symbol())?;
        } else if let Some(loc) = self.try_global_location() {
            // Use the global program as prefix only for .aleo programs.
            if with_session_globals(|sg| loc.program.as_str(sg, |s| s.ends_with(".aleo"))) {
                write!(f, "{}::", loc.program)?;
            }
        }

        // Qualifiers (always `::` separator, covers library names like `math_lib::Foo`).
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
            PathTarget::Local(sym) => write!(f, " [local: {sym}]"),
            PathTarget::Global(loc) => {
                write!(f, " [global: {loc}]")
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
