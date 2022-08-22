// Copyright (C) 2019-2022 Aleo Systems Inc.
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

pub mod inliner;
pub use inliner::*;

pub mod inline_expression;
pub use inline_expression::*;

pub mod inline_program;
pub use inline_program::*;

pub mod inline_statement;
pub use inline_statement::*;

use crate::{DiGraph, Pass, SymbolTable};

use leo_ast::{Ast, ProgramReconstructor};
use leo_errors::emitter::Handler;
use leo_errors::Result;
use leo_span::Symbol;

impl<'a> Pass for Inliner<'a> {
    type Input = (Ast, &'a Handler, &'a SymbolTable, &'a DiGraph<Symbol>);
    type Output = Result<Ast>;

    fn do_pass((ast, handler, st, call_graph): Self::Input) -> Self::Output {
        let mut reconstructor = Self::new(handler, st, call_graph);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err()?;

        Ok(Ast::new(program))
    }
}
