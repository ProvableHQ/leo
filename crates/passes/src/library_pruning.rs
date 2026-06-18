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

//! Drops library functions the program can never reach.
//!
//! `std` is injected into every program as a `FromLibrary` stub, so otherwise every compile drags
//! all ~6,000 of its functions through the backend before dead-code elimination discards the
//! unused ones. This pass removes library functions not transitively reachable from any
//! non-library function, using the call graph that type checking just built.
//!
//! This complements [`crate::DeadCodeEliminating`], which drops uncalled functions within a
//! program but is a no-op for libraries; library stubs are pruned here instead.

use crate::{CompilerState, Pass};

use leo_ast::{Ast, Library, Location, Stub};
use leo_errors::Result;

use indexmap::IndexSet;

/// Pass that prunes unreachable functions from `FromLibrary` stubs.
pub struct LibraryPruning;

impl Pass for LibraryPruning {
    type Input = ();
    type Output = ();

    const NAME: &str = "LibraryPruning";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        // This pass runs in the program backend, which a library build skips (frontend only, no
        // bytecode), so it never sees an `Ast::Library`. The guard skips programs that import no
        // library (e.g. `no_std`); otherwise every `FromLibrary` stub below is pruned, std included.
        let has_library_stub = matches!(&state.ast, Ast::Program(p)
            if p.stubs.values().any(|s| matches!(s, Stub::FromLibrary { .. })));
        if !has_library_stub {
            return Ok(());
        }

        // Reachability via a single multi-source traversal from every non-library function. Every
        // non-library function is a root, not just entry points, so a library function called from
        // any imported program survives even when the caller is itself unreachable.
        let mut reachable: IndexSet<Location> = IndexSet::new();
        let mut queue: Vec<Location> = Vec::new();
        for node in state.call_graph.nodes() {
            if !state.symbol_table.is_library(node.program) {
                queue.extend(state.call_graph.neighbors(node).cloned());
            }
        }
        while let Some(node) = queue.pop() {
            if reachable.insert(node.clone()) {
                queue.extend(state.call_graph.neighbors(&node).cloned());
            }
        }

        let Ast::Program(program) = &mut state.ast else { unreachable!("checked above") };
        for stub in program.stubs.values_mut() {
            if let Stub::FromLibrary { library, .. } = stub {
                prune_library(library, &reachable);
            }
        }

        Ok(())
    }
}

/// Removes functions not in `reachable` from `library` and its submodules. Structs, consts, and
/// interfaces are kept for now.
fn prune_library(library: &mut Library, reachable: &IndexSet<Location>) {
    let declares_const = !library.consts.is_empty() || library.modules.values().any(|module| !module.consts.is_empty());
    if declares_const {
        return;
    }

    let name = library.name;
    library.functions.retain(|(symbol, _)| reachable.contains(&Location::new(name, vec![*symbol])));

    for (path, module) in &mut library.modules {
        let program = module.unit_name;
        module.functions.retain(|(symbol, _)| {
            let mut full_path = Vec::with_capacity(path.len() + 1);
            full_path.extend_from_slice(path);
            full_path.push(*symbol);
            reachable.contains(&Location::new(program, full_path))
        });
    }
}
