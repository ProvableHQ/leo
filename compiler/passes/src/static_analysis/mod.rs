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

mod future_checker;

mod await_checker;
use self::await_checker::AwaitChecker;

mod expression;

mod statement;

mod program;

mod visitor;
use visitor::*;

use crate::Pass;

use leo_ast::ProgramVisitor;
use leo_errors::Result;
use leo_span::Symbol;

pub struct StaticAnalyzing;

impl Pass for StaticAnalyzing {
    type Input = ();
    type Output = ();

    const NAME: &str = "StaticAnalyzing";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = StaticAnalyzingVisitor {
            state,
            await_checker: AwaitChecker::new(),
            current_program: Symbol::intern(""),
            variant: None,
            non_async_external_call_seen: false,
        };
        visitor.visit_program(ast.as_repr());
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(())
    }
}
