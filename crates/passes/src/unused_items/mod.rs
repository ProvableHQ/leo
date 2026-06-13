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

//! Emits warnings for items that are never used. Coverage mirrors `rustc`'s
//! `dead_code` and `unused_variables` lints:
//!
//! - Unused functions (anything not externally callable — i.e. not a transition
//!   entry point or `view fn` — and not `@test`) — uses the holistic
//!   `call_count`. A leading `_` on the function name silences the warning (and,
//!   for top-level `Variant::Fn`, also forces inlining so the name never reaches
//!   the VM as a closure identifier).
//! - Unused structs — uses a local reachability graph built from composite
//!   members (records and user-referenced composites are roots).
//! - Unused `const` declarations — at every level (program scope, modules,
//!   library, function bodies). Consts are fully inlined at compile time and
//!   never reach the VM, so a `_X` prefix safely silences the warning.
//! - Unused local bindings — `let`, function/`final fn` parameters, and loop
//!   iteration variables that are never read.
//! - Unused imports — `import program.aleo;` declarations whose program is
//!   never referenced in a path.
//!
//! The pass runs in two phases, both driven by the standard `AstVisitor` /
//! `UnitVisitor` traits, with a strict collect-then-emit split:
//!
//! 1. [`collect::ReferenceCollector`] is pure analysis: it walks the entire AST
//!    and produces a [`CollectedUses`], building no diagnostics. It populates
//!    `used_imports` and `used_globals`; tracks lexical scopes and records the
//!    unused-local findings as each scope drains; and runs a reachability scan
//!    over the composite member-type graph to find the live composites (those
//!    reachable from records + any composite referenced from user code).
//! 2. [`emit::TopLevelItemChecker`] emits every warning: first it turns the
//!    phase-1 local findings into diagnostics, then — by walking the AST again
//!    without descending into function bodies — it warns on the unused top-level
//!    items and imports.
//!
//! Dead struct fields are tracked-but-not-warned: today the analysis is purely
//! syntactic (a field is "read" only when accessed via `MemberAccess`), which
//! produces too much noise on pass-through-only structs. A future refinement
//! (likely a data-flow-aware variant, plus an `@allow_unused` attribute) will
//! re-introduce that warning. Unreachable code after `return` is not handled
//! here — the type checker already rejects it with `ETYC0372025`.
//!
//! Only the compilation root (the user's own `Program` or `Library`) is
//! analyzed; imported programs/libraries are skipped because the same
//! dependency may be consumed differently by other compilations.

mod collect;
mod emit;

use crate::Pass;

use leo_ast::*;
use leo_errors::Result;
use leo_span::{Span, Symbol};

use indexmap::IndexSet;

pub struct UnusedItems;

impl Pass for UnusedItems {
    type Input = ();
    type Output = ();

    const NAME: &str = "UnusedItems";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);

        // Phase 1 — pure analysis: gather references, record unused-local findings, and derive
        // composite reachability. No diagnostics are built or emitted yet.
        let collected = {
            let mut collector = collect::ReferenceCollector::new(state);
            match &ast {
                Ast::Program(program) => collector.visit_program(program),
                Ast::Library(library) => collector.visit_library(library),
            }
            collector.into_data()
        };

        // Phase 2 — emit every warning: first the unused-locals from phase 1, then the unused
        // top-level items and imports found by walking the AST again (without bodies).
        {
            let mut checker = emit::TopLevelItemChecker::new(state, collected);
            checker.emit_local_warnings();
            match &ast {
                Ast::Program(program) => checker.visit_program(program),
                Ast::Library(library) => checker.visit_library(library),
            }
        }

        // Restore the AST before propagating any error so downstream passes don't see
        // `Ast::default()` if a future refinement starts emitting errors from this pass.
        state.ast = ast;
        state.handler.last_err()?;
        Ok(())
    }
}

/// True if the interned symbol's text starts with `_`, without allocating a `String`.
pub(crate) fn name_starts_with_underscore(name: Symbol) -> bool {
    leo_span::with_session_globals(|sg| name.as_str(sg, |s| s.starts_with('_')))
}

/// Everything phase 1 produces for phase 2 to consume. All derived data (including composite
/// reachability) is computed in [`collect::ReferenceCollector::into_data`]; phase 2 only emits.
pub(super) struct CollectedUses {
    pub(super) used_imports: IndexSet<Symbol>,
    pub(super) used_globals: IndexSet<Location>,
    /// Composites transitively reachable from the live roots (records / library top-level
    /// structs, plus any composite referenced from user code). A struct not in this set is
    /// unused.
    pub(super) live_composites: IndexSet<Location>,
    /// Unused-local findings recorded during the collect walk, in emission order. Phase 2
    /// turns these into diagnostics before warning on unused top-level items.
    pub(super) local_findings: Vec<LocalFinding>,
}

/// A locally-scoped name phase 1 flagged. Pure analysis data — phase 2 maps each to a
/// diagnostic via the [`crate::errors::unused_items`] builders.
pub(super) struct LocalFinding {
    pub(super) kind: LocalFindingKind,
    pub(super) name: Symbol,
    pub(super) span: Span,
}

pub(super) enum LocalFindingKind {
    /// A never-read `let`, parameter, or loop variable.
    UnusedVariable,
    /// A never-read local `const`.
    UnusedConst,
    /// A `_`-prefixed binding that was actually read, defeating the silencing marker.
    UsedUnderscore,
}
