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

//! The SSA Const Propagation pass propagates constant values through the program.
//! This pass runs after SSA formation, so each variable has a unique name.
//!
//! The pass tracks variables assigned literal values and replaces uses of those
//! variables with their constant values. It also forwards short-lived atom-only
//! aggregate fields, optional wrapper fields, and atom-only ternaries that can
//! be forwarded safely.

use crate::Pass;

use leo_ast::UnitReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
pub use visitor::SsaConstPropagationVisitor;

pub struct SsaConstPropagation;

pub struct SsaConstPropagationInput {
    pub forward_direct_composites: bool,
    pub propagate_constants: bool,
}

impl Default for SsaConstPropagationInput {
    fn default() -> Self {
        Self { forward_direct_composites: true, propagate_constants: true }
    }
}

impl Pass for SsaConstPropagation {
    type Input = SsaConstPropagationInput;
    type Output = ();

    const NAME: &str = "SsaConstPropagation";

    fn do_pass(input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        // Run the pass in a loop until no changes are made.
        for _ in 0..1024 {
            let ast = std::mem::take(&mut state.ast);
            let mut visitor = SsaConstPropagationVisitor {
                state,
                program: Symbol::intern(""),
                constants: Default::default(),
                atom_fielded_composites: Default::default(),
                aliases: Default::default(),
                forward_direct_composites: input.forward_direct_composites,
                propagate_constants: input.propagate_constants,
                ternaries: Default::default(),
                changed: false,
            };

            let ast = ast.map(
                |program| visitor.reconstruct_program(program),
                |library| library, // no-op for libraries
            );

            visitor.state.handler.last_err()?;
            visitor.state.ast = ast;

            // If no changes were made, we're done.
            if !visitor.changed {
                return Ok(());
            }
        }
        panic!("ran out of loops");
    }
}
