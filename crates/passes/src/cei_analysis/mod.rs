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

pub(crate) mod cross_layer_taint;
pub(crate) mod effect_summary;
pub(crate) mod finalize_visitor;

use crate::Pass;

use effect_summary::AutomatonState;

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
        let ast = std::mem::take(&mut state.ast);

        // Phase 0: Compute effect summaries bottom-up over the call graph.
        let effect_summaries = effect_summary::compute_effect_summaries(state);

        // Phase 1: Finalize-time CEI analysis.
        let mut cei_visitor = finalize_visitor::FinalizeCeiVisitor {
            state,
            current_program: Symbol::intern(""),
            effect_summaries,
            automaton_state: AutomatonState::BeforeInteraction,
            interaction_span: None,
            current_variant: None,
            in_finalize: false,
        };

        ast.visit(|program| cei_visitor.visit_program(program), |_library| {});

        // Phase 2: Cross-layer taint analysis.
        let mut taint_visitor = cross_layer_taint::CrossLayerTaintVisitor {
            state: cei_visitor.state,
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
