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
//! The pass tracks variables that are assigned literal values and replaces
//! uses of those variables with their constant values.

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
pub use visitor::SsaConstPropagationVisitor;

pub struct SsaConstPropagation;

impl Pass for SsaConstPropagation {
    type Input = ();
    type Output = ();

    const NAME: &str = "SsaConstPropagation";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        // Run the pass in a loop until no changes are made.
        for _ in 0..1024 {
            let mut ast = std::mem::take(&mut state.ast);
            let mut visitor = SsaConstPropagationVisitor {
                state,
                program: Symbol::intern(""),
                constants: Default::default(),
                changed: false,
            };
            ast.ast = visitor.reconstruct_program(ast.ast);
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
