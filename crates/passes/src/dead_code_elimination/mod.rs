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

//! The Dead Code Elimination pass traverses the AST and eliminates unused code,
//! specifically assignment statements, within the boundary of `transition`s and `function`s.
//! The pass is run after the Function Inlining pass.
//!
//! See https://en.wikipedia.org/wiki/Dead-code_elimination for more information.
//!
//! Consider the following flattened Leo code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$4$5 = value * value;
//!     $var$1 = $var$4$5;
//!     value$2 = $var$1;
//!     value$3 = $var$0 ? value$2 : value;
//!     value$6 = $var$1 * $var$1;
//!     return value$3;
//! }
//! ```
//!
//! The dead code elimination pass produces the following code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$4$5 = value * value;
//!     $var$1 = $var$4$5;
//!     value$2 = $var$1;
//!     value$3 = $var$0 ? value$2 : value;
//!     return value$3;
//! }
//! ```
//! Note this pass relies on the following invariants:
//! - No shadowing for all variables, composite names, function names, etc.
//! - Unique variable names (provided by SSA)
//! - Flattened code (provided by the flattening pass)

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct DeadCodeEliminatingOutput {
    pub statements_before: u32,
    pub statements_after: u32,
}

pub struct DeadCodeEliminating;

impl Pass for DeadCodeEliminating {
    type Input = ();
    type Output = DeadCodeEliminatingOutput;

    const NAME: &str = "DeadCodeEliminating";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = DeadCodeEliminatingVisitor {
            state,
            used_variables: Default::default(),
            program_name: Default::default(),
            statements_before: 0,
            statements_after: 0,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(DeadCodeEliminatingOutput {
            statements_before: visitor.statements_before,
            statements_after: visitor.statements_after,
        })
    }
}
