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

//! The destructuring pass traverses the AST and destructures tuples into individual variables.
//! This pass assumes that tuples have a depth of 1, which is ensured by the type checking pass.

use crate::Pass;

use leo_ast::ProgramReconstructor as _;

use leo_errors::Result;

mod expression;

mod program;

mod statement;

mod visitor;
use visitor::*;

/// A pass to rewrite tuple creation and accesses into other code.
///
/// This pass must be run after SSA, because it depends on identifiers being unique.
/// It must be run before flattening, because flattening cannot handle assignment statements.
pub struct Destructuring;

impl Pass for Destructuring {
    type Input = ();
    type Output = ();

    const NAME: &str = "Destructuring";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = DestructuringVisitor { state, tuples: Default::default(), is_async: false };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(())
    }
}
