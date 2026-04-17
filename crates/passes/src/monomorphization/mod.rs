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

//! Monomorphizes const generic functions and composites across every program reachable from the
//! top-level compilation unit.
//!
//! Each unique instantiation of a const generic function (`foo::[3u32]()`, `foo::[7u32]()`) or
//! composite (`Vec::[3u32]`, `Vec::[5u32]`) is replaced with a concrete specialization in which
//! the const parameters are substituted with their literal values. Specializations are given a
//! unique name (e.g. `foo::[3u32]`) and inserted alongside — or, when the original generic is no
//! longer referenced, in place of — their source definition.
//!
//! ### Example
//!
//! ```leo
//! transition main(x: u32, y: u32) -> u32 {
//!     return foo::[3u32]() + foo::[7u32]();
//! }
//!
//! inline foo::[N: u32]() -> u32 {
//!     return N;
//! }
//! ```
//!
//! Two concrete versions of `foo` are generated — one per const argument — and the original
//! generic `foo::[N]` is pruned once no unresolved call refers to it.
//!
//! ### Algorithm
//!
//! The pass is a single holistic walk, not a recursive per-stub pass. Conceptually:
//!
//! 1. **Seed** `function_map` and `composite_map` with every function and composite reachable
//!    from the top-level `Program` — its own scopes and modules, plus the contents of every
//!    `FromLeo`, `FromLibrary`, and `FromAleo` stub. Inserts from the current program come last
//!    so they override any stub placeholders.
//! 2. **Specialize composites** in post-order of the composite graph. A composite field may
//!    instantiate another generic composite, so callees must be monomorphized before their users.
//!    Specialized composites preserve their full module path (`types::Vec` → `types::Vec::[3u32]`)
//!    to prevent same-named structs in different submodules from colliding.
//! 3. **Specialize functions** via a post-order DFS over the call graph. Roots are every
//!    program's entry points, constructors, and non-generic top-level `fn`s — external roots
//!    included, so a generic closure called only from within an imported program still gets
//!    rewritten. `self.program` is set to each callee's own program during traversal so
//!    cross-program edges are interpreted from the callee's perspective.
//! 4. **Carry through** external definitions the DFS did not reach (they are still needed for
//!    stub assembly); drop current-program leftovers as dead code.
//! 5. **Assemble stubs** from the now-populated `reconstructed_*` maps. `FromLeo` stubs are
//!    rebuilt directly; `FromLibrary` stubs are reconstructed so their items pick up any
//!    monomorphized composite references.
//! 6. **Prune originals**: an original generic is removed once every call to it has been
//!    rewritten to a specialization. If unresolved calls remain, the original is kept so
//!    subsequent runs of this pass (inside the `ConstPropUnrollAndMorphing` fixed-point loop)
//!    can finish the job.

use crate::Pass;

use leo_ast::{CompositeExpression, CompositeType, ProgramReconstructor as _};
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
use visitor::*;

#[derive(Debug)]
pub struct MonomorphizationOutput {
    /// If we encountered calls to const generic functions that were not resolved, keep track of them in this vector
    pub unresolved_calls: Vec<leo_ast::CallExpression>,
    /// If we encountered const generic composite expressions that were not resolved, keep track of them in this vector
    pub unresolved_composite_exprs: Vec<CompositeExpression>,
    /// If we encountered const generic composite type instantiations that were not resolved, keep track of them in this
    /// vector
    pub unresolved_composite_types: Vec<CompositeType>,
    /// Did we change anything in this program?
    pub changed: bool,
}

pub struct Monomorphization;

impl Pass for Monomorphization {
    type Input = ();
    type Output = MonomorphizationOutput;

    const NAME: &str = "Monomorphization";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = MonomorphizationVisitor {
            state,
            program: Symbol::intern(""),
            reconstructed_functions: indexmap::IndexMap::new(),
            function_map: indexmap::IndexMap::new(),
            composite_map: indexmap::IndexMap::new(),
            monomorphized_functions: indexmap::IndexSet::new(),
            reconstructed_composites: indexmap::IndexMap::new(),
            unresolved_calls: Vec::new(),
            unresolved_composite_exprs: Vec::new(),
            unresolved_composite_types: Vec::new(),
            changed: false,
        };

        let ast = ast.map(
            |program| visitor.reconstruct_program(program),
            |library| library, // no-op for libraries
        );

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        Ok(MonomorphizationOutput {
            unresolved_calls: visitor.unresolved_calls,
            unresolved_composite_exprs: visitor.unresolved_composite_exprs,
            unresolved_composite_types: visitor.unresolved_composite_types,
            changed: visitor.changed,
        })
    }
}
