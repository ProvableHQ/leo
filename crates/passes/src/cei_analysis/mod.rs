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

//! The CEI analysis pass.
//!
//! Two independent sub-analyses share this module:
//!
//! - [`ordering`]: within a single finalize execution path, mutable-state
//!   reads and writes must precede any `Final::run()` interactions.
//! - [`cross_layer_taint`]: values produced in the proof-time transition
//!   body from external calls may be stale by finalize time.

pub(crate) mod cross_layer_taint;
pub(crate) mod ordering;

use crate::Pass;

use leo_ast::UnitVisitor;
use leo_errors::Result;
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct CeiAnalyzing;

impl Pass for CeiAnalyzing {
    type Input = ();
    type Output = ();

    const NAME: &str = "CeiAnalyzing";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        // Phase 1: CEI ordering (reads/writes vs. interactions in finalize).
        ordering::run(state);

        // Phase 2: Cross-layer taint (proof-time staleness reaching finalize).
        let ast = std::mem::take(&mut state.ast);
        let mut taint_visitor = cross_layer_taint::CrossLayerTaintVisitor {
            state,
            current_program: Symbol::intern(""),
            taint_map: IndexMap::new(),
            in_transition: false,
            implicit_taint: cross_layer_taint::TaintInfo::default(),
        };
        ast.visit(|program| taint_visitor.visit_program(program), |_library| {});
        taint_visitor.state.handler.last_err()?;
        taint_visitor.state.ast = ast;

        Ok(())
    }
}
