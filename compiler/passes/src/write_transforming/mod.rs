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

use crate::Pass;

use leo_ast::ProgramReconstructor as _;

use leo_errors::Result;

mod expression;

mod program;

mod statement;

mod visitor;
use visitor::*;

/// A pass to rewrite assignments to array accesses and struct accesses.
///
/// This pass makes variables for members of arrays and structs that are written to,
/// changes assignments to those members into assignments to those variables, and,
/// whenever the arrays or structs are accessed, reconstructs them from the variables.
/// So code like this:
///
/// let s = S { a: 1u8, b: [2u8, 3u8] };
/// s.a = 1u8;
/// s.b[0u8] = 4u8;
/// return s;
///
/// will be changed into something like this:
///
/// let s_a = 1u8;
/// let s_b_0 = 2u8;
/// let s_b_1 = 3u8;
/// s_b_1 = 4u8;
/// return S { a: s_a, b: [s_b_0, s_b_1] };
///
/// The pass requires that the AST is in SSA form (so that sub-expressions are always
/// variables or literals) and that tuples have been destructured.
/// Since the pass will create new assignments, `SsaForming` must be run again afterwards.
///
/// A note on the semantics of the language as implemented by this pass:
/// assignments and definitions in essence copy structs and arrays. Thus if we do
/// ```leo
/// let x = [0u8, 1u8];
/// let y = x;
/// y[0u8] = 12u8;
/// ```
/// x is still `[0u8, 1u8];`
pub struct WriteTransforming;

impl Pass for WriteTransforming {
    type Input = ();
    type Output = ();

    const NAME: &str = "WriteTransforming";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = WriteTransformingVisitor::new(state, ast.as_repr());
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(())
    }
}
