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

//! Iterators over the functions and composites defined in a `Program`, `Library`, or `Stub`,
//! plus `items_at_path` for filtering a `Location`-keyed map at a given module path.
//!
//! Used by passes like monomorphization, function inlining, and option lowering to seed their
//! lookup maps with every Leo-AST-backed definition reachable from the top-level program —
//! including those hidden behind `FromLeo` and `FromLibrary` stubs. `FromAleo` stubs are skipped
//! because their bodies live in Aleo bytecode and are not walkable as AST.

use indexmap::IndexMap;
use leo_ast::{Composite, Function, Library, Location, Program, Stub};
use leo_span::Symbol;

/// Collects items from `map` whose `Location` lies at `path_prefix` within `program` — i.e. the
/// item's path is exactly `path_prefix` followed by a single final segment. Yields
/// `(final_segment, value)` pairs. Pass `&[]` for top-level items.
///
/// For example, given `program = foo` and entries keyed by
/// `foo::bar`, `foo::baz::qux`, `foo::baz::quux`, `other::bar`:
/// - `path_prefix = &[]` yields `(bar, ...)` — only the `foo::bar` top-level entry.
/// - `path_prefix = &[baz]` yields `(qux, ...)` and `(quux, ...)` — items directly inside
///   `foo::baz`, not `other::bar` (wrong program) and not `foo::bar` (path differs).
pub fn items_at_path<'a, V: Clone + 'a>(
    map: &'a IndexMap<Location, V>,
    program: Symbol,
    path_prefix: &'a [Symbol],
) -> impl Iterator<Item = (Symbol, V)> + 'a {
    map.iter().filter_map(move |(loc, v)| {
        loc.path
            .split_last()
            .filter(|(_, rest)| *rest == path_prefix && loc.program == program)
            .map(|(last, _)| (*last, v.clone()))
    })
}

/// Yields `(Location, &Function)` for every function defined in `program`, including those in
/// nested modules. `program.stubs` is not traversed.
pub fn program_functions(program: &Program) -> impl Iterator<Item = (Location, &Function)> {
    let scopes = program.program_scopes.iter().flat_map(|(_, scope)| {
        let prog = scope.program_id.as_symbol();
        scope.functions.iter().map(move |(name, f)| (Location::new(prog, vec![*name]), f))
    });
    let modules = program.modules.iter().flat_map(module_functions);
    scopes.chain(modules)
}

/// Yields `(Location, &Composite)` for every composite defined in `program`, including those in
/// nested modules.
pub fn program_composites(program: &Program) -> impl Iterator<Item = (Location, &Composite)> {
    let scopes = program.program_scopes.iter().flat_map(|(_, scope)| {
        let prog = scope.program_id.as_symbol();
        scope.composites.iter().map(move |(name, c)| (Location::new(prog, vec![*name]), c))
    });
    let modules = program.modules.iter().flat_map(module_composites);
    scopes.chain(modules)
}

/// Yields `(Location, &Function)` for every function defined in `library`, including those in
/// nested modules.
pub fn library_functions(library: &Library) -> impl Iterator<Item = (Location, &Function)> {
    let name = library.name;
    let top = library.functions.iter().map(move |(sym, f)| (Location::new(name, vec![*sym]), f));
    let modules = library.modules.iter().flat_map(module_functions);
    top.chain(modules)
}

/// Yields `(Location, &Composite)` for every composite defined in `library`, including those in
/// nested modules.
pub fn library_composites(library: &Library) -> impl Iterator<Item = (Location, &Composite)> {
    let name = library.name;
    let top = library.structs.iter().map(move |(sym, c)| (Location::new(name, vec![*sym]), c));
    let modules = library.modules.iter().flat_map(module_composites);
    top.chain(modules)
}

/// Yields `(Location, &Function)` for every function defined in `stub` whose body is available as
/// Leo AST. `FromAleo` stubs are skipped because their bodies live in Aleo bytecode.
pub fn stub_functions(stub: &Stub) -> Box<dyn Iterator<Item = (Location, &Function)> + '_> {
    match stub {
        Stub::FromLeo { program, .. } => Box::new(program_functions(program)),
        Stub::FromLibrary { library, .. } => Box::new(library_functions(library)),
        Stub::FromAleo { .. } => Box::new(std::iter::empty()),
    }
}

/// Yields `(Location, &Composite)` for every composite defined in `stub` whose body is available
/// as Leo AST. `FromAleo` stubs are skipped.
pub fn stub_composites(stub: &Stub) -> Box<dyn Iterator<Item = (Location, &Composite)> + '_> {
    match stub {
        Stub::FromLeo { program, .. } => Box::new(program_composites(program)),
        Stub::FromLibrary { library, .. } => Box::new(library_composites(library)),
        Stub::FromAleo { .. } => Box::new(std::iter::empty()),
    }
}

fn module_functions<'a>(
    (path, m): (&'a Vec<Symbol>, &'a leo_ast::Module),
) -> impl Iterator<Item = (Location, &'a Function)> {
    m.functions.iter().map(move |(name, f)| {
        let full: Vec<Symbol> = path.iter().copied().chain(std::iter::once(*name)).collect();
        (Location::new(m.unit_name, full), f)
    })
}

fn module_composites<'a>(
    (path, m): (&'a Vec<Symbol>, &'a leo_ast::Module),
) -> impl Iterator<Item = (Location, &'a Composite)> {
    m.composites.iter().map(move |(name, c)| {
        let full: Vec<Symbol> = path.iter().copied().chain(std::iter::once(*name)).collect();
        (Location::new(m.unit_name, full), c)
    })
}
