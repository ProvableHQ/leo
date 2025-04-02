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

use leo_ast::ProgramReconstructor;
use leo_errors::Result;
use leo_span::{Span, Symbol};

mod duplicate;

mod range_iterator;
use range_iterator::*;

mod statement;

mod program;

mod visitor;
use visitor::*;

pub struct UnrollingOutput {
    /// If we encountered a loop that was not unrolled, here's it's span.
    pub loop_not_unrolled: Option<Span>,
    /// Did we unroll any loop?
    pub loop_unrolled: bool,
}

pub struct Unrolling;

impl Pass for Unrolling {
    type Input = ();
    type Output = UnrollingOutput;

    const NAME: &str = "Unrolling";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor =
            UnrollingVisitor { state, program: Symbol::intern(""), loop_not_unrolled: None, loop_unrolled: false };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(UnrollingOutput { loop_not_unrolled: visitor.loop_not_unrolled, loop_unrolled: visitor.loop_unrolled })
    }
}
