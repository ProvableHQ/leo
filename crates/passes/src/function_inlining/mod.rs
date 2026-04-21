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

//! Inlines function calls at their call sites across every program reachable from the top-level
//! compilation unit.
//!
//! Aleo `call` can target either a `function` (entry point) or a `closure`, in the callee's own
//! program or another program. So top-level `Variant::Fn` closures can be emitted as standalone
//! Aleo closures and called across programs just like entry points — the heuristics in step 4
//! below decide whether to inline them or keep them as closures. What must always be inlined is
//! anything Aleo's bytecode can't represent as a top-level closure: submodule functions (Aleo
//! resources are flat identifiers, not paths), library functions (libraries aren't deployable),
//! and `final fn`s (on-chain-only code that can't be called via `call`).
//!
//! See <https://en.wikipedia.org/wiki/Inline_expansion> for background on inlining in general.
//!
//! ### Example
//!
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$1 = foo(value);
//!     value$2 = $var$1;
//!     value$3 = $var$0 ? value$2 : value;
//!     return value$3;
//! }
//!
//! inline foo(x: u8) -> u8 {
//!     $var$4 = x * x;
//!     return $var$4;
//! }
//! ```
//!
//! produces
//!
//! ```leo
//! inline foo(x: u8) -> u8 {
//!     $var$4 = x * x;
//!     return $var$4;
//! }
//!
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$4$5 = value * value;
//!     $var$1 = $var$4$5;
//!     value$2 = $var$1;
//!     value$3 = $var$0 ? value$2 : value;
//!     return value$3;
//! }
//! ```
//!
//! ### Algorithm
//!
//! The pass runs in two phases. Monomorphization is expected to have already specialized every
//! const generic, so this pass sees only regular parameters.
//!
//! **Phase 1 — Analysis.** `AnalysisVisitor` walks the AST and records every function called
//! from a `Fn`/`FinalFn`/`Finalize` body or a constructor. The resulting `always_inline` set
//! force-inlines any function that reaches an on-chain context even transitively.
//!
//! **Phase 2 — Transform.**
//!
//! 1. **Seed** `function_map` with every function reachable from the top-level `Program` — its
//!    own scopes and modules plus the contents of every `FromLeo` and `FromLibrary` stub.
//!    Current-program inserts come last so they override any stub placeholders. `FromAleo`
//!    stubs are skipped; their bodies live in Aleo bytecode and are not available to inline.
//! 2. **Walk** the call graph in post-order so each callee is already reconstructed when its
//!    callers are visited. `self.program` is set to each callee's own program during traversal
//!    so `reconstruct_call` interprets cross-program edges from the callee's perspective.
//! 3. **At each call site**, emit a direct Aleo `call` if the callee is a cross-program entry
//!    point (inlining would lose its transition semantics) — otherwise consult step 4. When
//!    inlining is chosen, substitute parameters for arguments via `Replacer` and run
//!    `SsaFormingVisitor` with `rename_defs` enabled so inlined definitions don't shadow caller
//!    bindings. The call expression is replaced with the inlined block's return value (or a
//!    unit expression if there was none).
//! 4. **Force-inline** if any of: the callee is a submodule function (same program or cross-
//!    program — Aleo resources are flat identifiers), a library function, a `final fn`, a `Fn`
//!    called from an on-chain context, a `Fn` taking more than 16 args, or a `Fn` whose
//!    signature names any optional types. Otherwise inline conditionally when the callee has a
//!    single caller, no args, or only empty-typed args — a top-level `Variant::Fn` that doesn't
//!    trip any of those heuristics stays as a closure and is invoked via `call`, whether the
//!    call site is in the same program or a different one. An `@no_inline` annotation
//!    suppresses the conditional cases and emits a warning if a mandatory rule would have
//!    overridden it.
//! 5. **Carry through** external definitions the DFS did not process so `FromLeo` stub
//!    assembly can pick them up; drop current-program leftovers as dead code.
//! 6. **Assemble stubs** from `reconstructed_functions`. Top-level `Variant::Fn`s are kept in
//!    the stub — same-program callers (inside the stub's own entry points) and cross-program
//!    callers (in the compilation unit) both emit direct `call`s into them, so removing them
//!    would leave dangling references in the emitted bytecode.

use crate::Pass;

use analysis::AnalysisVisitor;
use indexmap::IndexMap;
use leo_ast::{ProgramReconstructor, ProgramVisitor};
use leo_errors::Result;
use leo_span::Symbol;
use transform::TransformVisitor;

mod analysis;
mod transform;

pub struct FunctionInlining;

impl Pass for FunctionInlining {
    type Input = ();
    type Output = ();

    const NAME: &str = "FunctionInlining";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        // Phase 1: Analysis - collect functions that ought always be inlined
        let mut analyzer = AnalysisVisitor::new();
        state.ast.visit(
            |program| analyzer.visit_program(program),
            |_library| {
                // no-op for libraries
            },
        );

        // Phase 2: Transformation - convert Function to Inline where needed
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = TransformVisitor {
            state,
            reconstructed_functions: IndexMap::new(),
            program: Symbol::intern(""),
            function_map: IndexMap::new(),
            is_onchain: false,
            always_inline: analyzer.functions_to_inline,
        };

        let ast = ast.map(
            |program| visitor.reconstruct_program(program),
            |library| library, // no-op for libraries
        );

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        Ok(())
    }
}
