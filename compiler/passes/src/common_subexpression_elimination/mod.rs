// Copyright (C) 2019-2025 Provable Inc.
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

//! The common subexpression elimination pass traverses the AST and removes
//! duplicate definitions.
//!
//! That is, this code:
//! ```leo
//! function main(val: field) -> field {
//!     let x = val + 1field;
//!     let y = val + 1field;
//!     let z = x + y;
//!     return z;
//! }
//! ```
//!
//! Will be transformed to something like this:
//! ```leo
//! function main(val: field) -> field {
//!     let x = val + 1field;
//!     let z = x + x;
//!     return z;
//! }
//! ```
//!
//! The pass expects flattening and destructuring to have already been run, and
//! for the code to be in SSA form. Given that there is little flow control
//! at this point in the compiler, there's no need for any kind of data flow analysis.

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct CommonSubexpressionEliminating;

impl Pass for CommonSubexpressionEliminating {
    type Input = ();
    type Output = ();

    const NAME: &str = "CommonSubexpressionEliminating";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = CommonSubexpressionEliminatingVisitor { state, scopes: Default::default() };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}
